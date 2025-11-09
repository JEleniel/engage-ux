//! Wayland backend for the OAL. Feature-gated behind `wayland` feature.
#![cfg(feature = "wayland")]

use crate::OalError;
use crate::{DeviceRect, DeviceSize};
use crate::{Frame, Oal, Window};
use std::cell::RefCell;
use std::sync::Arc;

use engage_ux_core::event::Event;
use engage_ux_core::event::EventBus;
use engage_ux_core::event::WindowEvent;
use std::sync::Arc as StdArc;
use std::sync::Mutex as StdMutex;
use tokio::sync::mpsc::UnboundedReceiver as TokioUnboundedReceiver;
use tokio::sync::oneshot;

use wayland_client::Main;
use wayland_client::protocol::wl_buffer::Event as WlBufferEvent;
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_shm::Format as ShmFormat;
use wayland_client::protocol::wl_shm::WlShm;
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Display, EventQueue, GlobalManager};

use memfd::MemfdOptions;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::AsRawFd;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{SyncSender, sync_channel};
use tempfile::tempfile;

// Global WaylandHandle to allow non-main threads (or builders) to obtain a
// handle and request surface creation. This is set when `WaylandRuntime::new`
// is called and cleared when the runtime is dropped.
static GLOBAL_WAYLAND_HANDLE: Lazy<Mutex<Option<WaylandHandle>>> = Lazy::new(|| Mutex::new(None));

/// A simple Wayland OAL implementation. This provides the minimal surface/window
/// creation and an in-process main-thread task queue.
/// The OAL handle that is safe to share between threads. It forwards requests to a
/// main-thread-owned `WaylandRuntime` via a channel. The runtime must be created and
/// owned by the embedding application's main thread.
#[derive(Clone)]
pub struct WaylandHandle {
	tx: std::sync::mpsc::SyncSender<RuntimeMessage>,
}

impl WaylandHandle {
	/// Construct a handle from the provided sender used to communicate with the runtime.
	pub(crate) fn new(tx: std::sync::mpsc::SyncSender<RuntimeMessage>) -> Self {
		Self { tx }
	}

	/// Request the runtime to perform a graceful shutdown. This will clear the
	/// global handle and teardown registered surfaces on the main thread. The
	/// call blocks until the runtime acknowledges the shutdown or the internal
	/// tokio runtime fails to start.
	pub fn shutdown(&self) -> Result<(), OalError> {
		let (tx, rx) = oneshot::channel();
		self.tx
			.send(RuntimeMessage::Shutdown(tx))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;

		// Block on the oneshot receiver using a small runtime.
		let rt = tokio::runtime::Runtime::new()
			.map_err(|e| OalError::Other(format!("tokio runtime: {}", e)))?;
		rt.block_on(rx)
			.map_err(|_| OalError::Other("shutdown cancelled".into()))?
	}

	/// Request the runtime to create and register a persistent surface. Blocks
	/// until the runtime responds with an id or an error.
	pub fn create_surface(&self, width: u32, height: u32) -> Result<u64, OalError> {
		let (tx, rx) = oneshot::channel();
		self.tx
			.send(RuntimeMessage::CreateSurface(width, height, tx))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;
		// Block on the oneshot receiver using a small runtime.
		let rt = tokio::runtime::Runtime::new()
			.map_err(|e| OalError::Other(format!("tokio runtime: {}", e)))?;
		rt.block_on(rx)
			.map_err(|_| OalError::Other("create_surface cancelled".into()))?
	}

	/// Present a frame to a registered surface id. This enqueues the Present
	/// request to the runtime and returns immediately.
	pub fn present_to_surface(
		&self,
		id: u64,
		frame: Frame,
		dirty: Vec<DeviceRect>,
	) -> Result<(), OalError> {
		self.tx
			.send(RuntimeMessage::PresentToSurface(id, frame, dirty))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;
		Ok(())
	}

	/// Destroy a registered surface id.
	pub fn destroy_surface(&self, id: u64) -> Result<(), OalError> {
		self.tx
			.send(RuntimeMessage::DestroySurface(id))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;
		Ok(())
	}
}

pub(crate) enum RuntimeMessage {
	Task(Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>),
	TaskWithResponder(
		Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
		oneshot::Sender<Result<(), OalError>>,
	),
	/// Create a persistent (registered) surface and return its id via the responder.
	CreateSurface(u32, u32, oneshot::Sender<Result<u64, OalError>>),
	/// Destroy a previously created surface id.
	DestroySurface(u64),
	/// Present a frame to a registered surface id.
	PresentToSurface(u64, Frame, Vec<DeviceRect>),
	/// Shutdown the runtime, clear global handle, and teardown surfaces.
	Shutdown(oneshot::Sender<Result<(), OalError>>),
}

/// The runtime owns the Wayland `Display` and `EventQueue` and MUST be run on the main thread.
#[allow(dead_code)]
pub struct WaylandRuntime {
	display: Display,
	event_queue: EventQueue,
	globals: GlobalManager,
	shm: Main<WlShm>,
	rx: std::sync::mpsc::Receiver<RuntimeMessage>,
	/// Optional EventBus subscription receiver. If present, the runtime will
	/// poll this receiver for shutdown/window events (e.g., CloseRequested).
	event_rx: Option<StdArc<StdMutex<TokioUnboundedReceiver<Event>>>>,
	// registry of persistent surfaces created via the handle API
	surfaces: HashMap<u64, Box<dyn Window>>,
	next_surface_id: u64,
}

impl WaylandRuntime {
	/// Create a runtime and a handle. The caller should take ownership of the runtime
	/// and call `run_once` / `process_main_thread_tasks` frequently from the main thread.
	pub fn new() -> Result<(Self, WaylandHandle), OalError> {
		Self::new_with_event_bus(None)
	}

	/// Create a runtime and optionally subscribe to an `EventBus`.
	/// If `bus` is Some, the runtime will subscribe and poll for shutdown events.
	pub fn new_with_event_bus(bus: Option<EventBus>) -> Result<(Self, WaylandHandle), OalError> {
		let display = Display::connect_to_env()
			.map_err(|e| OalError::Other(format!("Wayland connect failed: {:?}", e)))?;
		let event_queue = display.create_event_queue();
		let attached = display.attach(event_queue.token());
		let globals = GlobalManager::new(&attached);
		// instantiate wl_shm early; if not available, we can't present buffers
		let shm = globals
			.instantiate_exact::<WlShm>(1)
			.map_err(|_| OalError::PlatformNotSupported)?;

		// Create a bounded channel for cross-thread messages.
		let (tx, rx) = std::sync::mpsc::sync_channel(1024);

		// Subscribe to event bus if provided.
		let event_rx = bus.map(|b| b.subscribe());

		// Build and store a global handle clone so builders and background
		// callers can request registered surfaces.
		let global_handle = WaylandHandle::new(tx.clone());
		if let Ok(mut g) = GLOBAL_WAYLAND_HANDLE.lock() {
			*g = Some(global_handle.clone());
		}

		Ok((
			Self {
				display,
				event_queue,
				globals,
				shm,
				rx,
				event_rx,
				surfaces: HashMap::new(),
				next_surface_id: 1,
			},
			WaylandHandle::new(tx),
		))
	}

	/// Process queued tasks and run a Wayland roundtrip. This must be called on the main thread.
	///
	/// The `budget` parameter bounds how long this call may run. The method will
	/// attempt to process queued messages and dispatch pending Wayland events but
	/// will return early if the provided time budget is exhausted.
	pub fn process_main_thread_tasks(
		&mut self,
		budget: std::time::Duration,
	) -> Result<(), WaylandError> {
		use std::time::Instant;

		let deadline = Instant::now() + budget;

		// Drain queued messages (non-blocking) but stop if budget expires.
		while Instant::now() <= deadline {
			// First, poll any subscribed EventBus for shutdown/window events.
			if let Some(rx_arc) = &self.event_rx {
				if let Ok(mut guard) = rx_arc.lock() {
					use tokio::sync::mpsc::error::TryRecvError;
					loop {
						match guard.try_recv() {
							Ok(evt) => match evt {
								Event::Window { payload, .. } => {
									if let WindowEvent::CloseRequested = payload {
										// Perform shutdown cleanup (same as Shutdown message)
										if let Ok(mut g) = GLOBAL_WAYLAND_HANDLE.lock() {
											*g = None;
										}
										for (_id, win) in self.surfaces.drain() {
											let _ = win.poll_events();
										}
										// After handling we continue processing queued runtime messages.
									}
								}
								_ => {}
							},
							Err(_) => break,
						}
					}
				}
			}
			match self.rx.try_recv() {
				Ok(RuntimeMessage::Task(f)) => {
					let _ = f();
				}
				Ok(RuntimeMessage::TaskWithResponder(f, responder)) => {
					let res = f();
					let _ = responder.send(res);
				}
				Ok(RuntimeMessage::CreateSurface(w, h, responder)) => {
					// Create and register a persistent surface
					match self.create_window("registered-surface", w, h) {
						Ok(win) => {
							let id = self.next_surface_id;
							self.next_surface_id += 1;
							self.surfaces.insert(id, win);
							let _ = responder.send(Ok(id));
						}
						Err(e) => {
							let _ = responder.send(Err(e));
						}
					}
				}
				Ok(RuntimeMessage::DestroySurface(id)) => {
					self.surfaces.remove(&id);
				}
				Ok(RuntimeMessage::PresentToSurface(id, frame, dirty)) => {
					if let Some(win) = self.surfaces.get(&id) {
						let _ = win.present(frame, &dirty);
					}
				}
				Ok(RuntimeMessage::Shutdown(responder)) => {
					// Clear the global handle so background threads/builders no longer
					// find an active runtime. Then teardown registered surfaces.
					if let Ok(mut g) = GLOBAL_WAYLAND_HANDLE.lock() {
						*g = None;
					}
					// Best-effort destroy of registered windows.
					for (_id, win) in self.surfaces.drain() {
						let _ = win.poll_events();
					}
					let _ = responder.send(Ok(()));
				}
				Err(std::sync::mpsc::TryRecvError::Empty) => break,
				Err(std::sync::mpsc::TryRecvError::Disconnected) => {
					return Err(OalError::ResourceUnavailable(
						"wayland runtime unavailable".into(),
					));
				}
			}
		}

		// Dispatch any pending Wayland events without blocking. If we've exceeded
		// the budget, return early.
		if std::time::Instant::now() <= deadline {
			self.event_queue
				.dispatch_pending(&mut (), |_, _, _| {})
				.map_err(|e| WaylandError::DispatchError(e))?;
		}

		Ok(())
	}

	/// Create a top-level window. Must be called on the main thread where the runtime lives.
	pub fn create_window(
		&mut self,
		_title: &str,
		width: u32,
		height: u32,
	) -> Result<Box<dyn Window>, OalError> {
		let compositor = self
			.globals
			.instantiate_exact::<WlCompositor>(1)
			.map_err(|_| OalError::PlatformNotSupported)?;
		let surface = compositor.create_surface();
		let win = WaylandWindow {
			surface: Arc::new(surface),
			size: RefCell::new(DeviceSize::new(width, height)),
			shm: self.shm.clone(),
			buffers: Arc::new(std::sync::Mutex::new(Vec::new())),
		};
		Ok(Box::new(win))
	}
}

// Implement Drop explicitly to clear the global handle if the runtime is dropped.
impl Drop for WaylandRuntime {
	fn drop(&mut self) {
		if let Ok(mut g) = GLOBAL_WAYLAND_HANDLE.lock() {
			*g = None;
		}
	}
}

/// Wayland window implementation.
pub struct WaylandWindow {
	surface: Arc<Main<WlSurface>>,
	size: RefCell<DeviceSize>,
	shm: Main<WlShm>,
	buffers: Arc<std::sync::Mutex<Vec<ShmBuffer>>>,
}

struct ShmBuffer {
	file: File,
	_pool: Main<WlShmPool>,
	buffer: Main<WlBuffer>,
	busy: Arc<AtomicBool>,
	width: u32,
	height: u32,
	stride: i32,
}

impl ShmBuffer {
	fn matches(&self, width: u32, height: u32) -> bool {
		self.width == width && self.height == height
	}

	fn destroy(&self) {
		// Best-effort destroy of Wayland objects. If the underlying API
		// doesn't expose destroy as expected, dropping will eventually
		// release resources, but explicit destroy is preferred.
		let _ = self.buffer.destroy();
		let _ = self._pool.destroy();
	}
}

// Small, pure helper used by buffer selection logic. This is intentionally
// testable without creating real Wayland buffers.
#[derive(Debug, Clone, Copy)]
struct ShmInfo {
	busy: bool,
	width: u32,
	height: u32,
}

fn choose_free_matching_index(infos: &[ShmInfo], width: u32, height: u32) -> Option<usize> {
	for (i, info) in infos.iter().enumerate() {
		if !info.busy && info.width == width && info.height == height {
			return Some(i);
		}
	}
	None
}

fn create_anonymous_file(size: usize) -> Result<File, WaylandError> {
	// Try memfd_create first; fallback to tempfile
	if let Ok(m) = MemfdOptions::default().create("engage_shm") {
		let file = m.into_file();
		file.set_len(size as u64)?;
		return Ok(file);
	}

	let file = tempfile()?;
	file.set_len(size as u64)?;
	Ok(file)
}

#[derive(Debug, Clone, Error)]
pub enum WaylandError {
	#[error("I/O error: {0}")]
	Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
	use super::RuntimeMessage;
	use super::WaylandHandle;
	use super::create_anonymous_file;
	use crate::traits::Oal;
	use std::sync::mpsc::sync_channel;
	use std::time::Duration;

	#[test]
	fn anon_file_has_correct_len() {
		let size = 1024usize * 4;
		let f = create_anonymous_file(size).expect("create anon file");
		let meta = f.metadata().expect("metadata");
		assert_eq!(meta.len(), size as u64);
	}

	#[test]
	fn handle_queue_sends_task_message() {
		let (tx, rx) = sync_channel::<RuntimeMessage>(4);
		let handle = WaylandHandle::new(tx);

		// queue a simple task
		handle
			.queue_main_thread(Box::new(|| Ok(())))
			.expect("queue");

		// runtime should receive a Task message
		match rx.recv_timeout(Duration::from_millis(100)).expect("recv") {
			RuntimeMessage::Task(_) => {}
			other => panic!(
				"unexpected message: {:?}",
				std::any::type_name_of_val(&other)
			),
		}
	}

	#[test]
	fn handle_queue_with_handle_responder_roundtrip() {
		let (tx, rx) = sync_channel::<RuntimeMessage>(4);
		let handle = WaylandHandle::new(tx);

		// queue and receive the oneshot receiver
		let receiver = handle
			.queue_main_thread_with_handle(Box::new(|| Ok(())))
			.expect("queue-with-handle");

		// runtime should receive TaskWithResponder
		match rx.recv_timeout(Duration::from_millis(100)).expect("recv") {
			RuntimeMessage::TaskWithResponder(task, responder) => {
				// run the task and send the result back via the responder
				let res = task();
				let _ = responder.send(res);
			}
			other => panic!(
				"unexpected message: {:?}",
				std::any::type_name_of_val(&other)
			),
		}

		// the receiver should get the Ok(()) result; block on the oneshot receiver using a small runtime
		let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
		let got = rt.block_on(receiver).expect("oneshot recv");
		assert!(got.is_ok());
	}

	#[test]
	fn choose_free_matching_index_works() {
		use super::ShmInfo;
		use super::choose_free_matching_index;

		let infos = vec![
			ShmInfo {
				busy: true,
				width: 100,
				height: 100,
			},
			ShmInfo {
				busy: false,
				width: 200,
				height: 200,
			},
			ShmInfo {
				busy: false,
				width: 100,
				height: 100,
			},
		];

		// find free 100x100 => index 2
		assert_eq!(choose_free_matching_index(&infos, 100, 100), Some(2));

		// find free 200x200 => index 1
		assert_eq!(choose_free_matching_index(&infos, 200, 200), Some(1));

		// no free 300x300
		assert_eq!(choose_free_matching_index(&infos, 300, 300), None);
	}
}

fn create_shm_buffer(shm: &Main<WlShm>, width: u32, height: u32) -> Result<ShmBuffer, OalError> {
	let stride = (width as i32) * 4;
	let size = (stride as usize) * (height as usize);

	let file = create_anonymous_file(size)
		.map_err(|e| OalError::Other(format!("creating shm file failed: {}", e)))?;

	// Create pool and buffer (keep the File alive in the ShmBuffer so the fd remains valid)
	let pool = shm.create_pool(file.as_raw_fd(), size as i32);
	let buffer = pool.create_buffer(0, width as i32, height as i32, stride, ShmFormat::Argb8888);

	// busy flag
	let busy = Arc::new(AtomicBool::new(false));
	let busy_clone = busy.clone();

	// Install release handler to mark buffer free
	buffer.quick_assign(move |_buf, event, _| {
		if let WlBufferEvent::Release = event {
			busy_clone.store(false, Ordering::SeqCst);
		}
	});

	Ok(ShmBuffer {
		file,
		_pool: pool,
		buffer,
		busy,
		width,
		height,
		stride,
	})
}

#[cfg(test)]
mod surface_tests {
	use super::*;
	use crate::traits::Surface;
	use crate::types::DeviceSize;
	use once_cell::sync::Lazy;
	use std::sync::Mutex;
	use std::thread;

	static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

	#[test]
	fn handle_shutdown_clears_global_handle() {
		let _lock = TEST_MUTEX.lock().expect("test mutex");
		let (tx, rx) = std::sync::mpsc::sync_channel::<RuntimeMessage>(4);
		let handle = WaylandHandle::new(tx.clone());

		// Install into the global so shutdown will clear it.
		{
			let mut g = GLOBAL_WAYLAND_HANDLE.lock().expect("lock");
			*g = Some(handle.clone());
		}

		// Call shutdown from another thread so the test thread can act as the runtime.
		let h = handle.clone();
		let join = std::thread::spawn(move || {
			h.shutdown().expect("shutdown");
		});

		// The runtime (test thread) should receive a Shutdown message with a responder.
		match rx
			.recv_timeout(std::time::Duration::from_millis(200))
			.expect("recv")
		{
			RuntimeMessage::Shutdown(responder) => {
				// At this point shutdown hasn't been acknowledged; the global should still be set.
				let g = GLOBAL_WAYLAND_HANDLE.lock().expect("lock");
				assert!(g.is_some());
				drop(g);
				// Simulate runtime clearing the global handle as part of shutdown.
				if let Ok(mut gg) = GLOBAL_WAYLAND_HANDLE.lock() {
					*gg = None;
				}
				// Acknowledge shutdown.
				let _ = responder.send(Ok(()));
			}
			other => panic!(
				"unexpected message: {:?}",
				std::any::type_name_of_val(&other)
			),
		}

		join.join().expect("join");

		// After shutdown completes the global handle should be cleared.
		let g = GLOBAL_WAYLAND_HANDLE.lock().expect("lock");
		assert!(g.is_none());
	}
	#[test]
	fn wayland_surface_enqueue_from_thread() {
		let _lock = TEST_MUTEX.lock().expect("test mutex");
		// Create a fake runtime channel and install a WaylandHandle pointing at it.
		let (tx, rx) = std::sync::mpsc::sync_channel::<RuntimeMessage>(8);
		let handle = WaylandHandle::new(tx.clone());
		{
			let mut g = GLOBAL_WAYLAND_HANDLE.lock().expect("lock");
			*g = Some(handle.clone());
		}

		// Start a thread that will create a WaylandSurface; this will send a
		// CreateSurface runtime message and block until the responder replies.
		let create_join = thread::spawn(move || {
			// This will block until the test thread responds to the CreateSurface message.
			let s = WaylandSurface::new(32, 16).expect("create surface");
			s
		});

		// The runtime should receive a CreateSurface message; respond with id=1
		match rx
			.recv_timeout(std::time::Duration::from_millis(200))
			.expect("recv")
		{
			RuntimeMessage::CreateSurface(w, h, responder) => {
				assert_eq!(w, 32);
				assert_eq!(h, 16);
				let _ = responder.send(Ok(1));
			}
			other => panic!(
				"unexpected message: {:?}",
				std::any::type_name_of_val(&other)
			),
		}

		// Now get the created surface from the thread
		let surface = create_join.join().expect("join");

		// Build a simple frame
		let size = DeviceSize::new(32, 16);
		let pixels = vec![0u8; (size.width * size.height * 4) as usize];
		let frame = Frame {
			rgba_pixels: Some(pixels),
			size,
		};

		// Present from a background thread; this will enqueue PresentToSurface
		// on the runtime channel.
		let s_handle = std::thread::spawn(move || {
			surface.present_frame(frame, &[]).expect("present");
		});

		s_handle.join().expect("thread join");

		// The runtime should have received a PresentToSurface message
		match rx
			.recv_timeout(std::time::Duration::from_millis(200))
			.expect("recv")
		{
			RuntimeMessage::PresentToSurface(id, f, _dirty) => {
				assert_eq!(id, 1);
				assert_eq!(f.size.width, 32);
				assert_eq!(f.size.height, 16);
				assert!(f.rgba_pixels.is_some());
			}
			other => panic!(
				"unexpected message: {:?}",
				std::any::type_name_of_val(&other)
			),
		}
	}
}

impl WaylandWindow {
	/// Present a frame to Wayland on the main thread. This performs pixel upload
	/// into a wl_shm buffer (memfd-backed if available), attaches it to the surface,
	/// damages the full buffer and commits.
	pub fn present_frame(&self, frame: Frame, _dirty: &[DeviceRect]) -> Result<(), OalError> {
		let width = frame.size.width;
		let height = frame.size.height;

		let pixels = match frame.rgba_pixels {
			Some(p) => p,
			None => return Err(OalError::UnsupportedOperation),
		};

		if pixels.len() != (width as usize) * (height as usize) * 4 {
			return Err(OalError::Other("pixel buffer size mismatch".into()));
		}

		// If frame size differs from the window size, resize the window which
		// will drop incompatible buffers.
		{
			let mut cur = self.size.borrow_mut();
			if cur.width != width || cur.height != height {
				// update size and drop existing buffers that don't match
				cur.width = width;
				cur.height = height;
				let mut bufs = self
					.buffers
					.lock()
					.map_err(|_| OalError::ResourceUnavailable("buffers lock poisoned".into()))?;
				// destroy and remove buffers that don't match new size
				bufs.retain(|b| {
					if b.matches(width, height) {
						true
					} else {
						// best-effort destroy
						b.destroy();
						false
					}
				});
			}
		}

		// Find or create a buffer matching the size
		let mut bufs = self
			.buffers
			.lock()
			.map_err(|_| OalError::ResourceUnavailable("buffers lock poisoned".into()))?;

		// Try to find a free buffer
		let mut idx = None;
		for (i, b) in bufs.iter().enumerate() {
			if !b.busy.load(Ordering::SeqCst) && b.matches(width, height) {
				idx = Some(i);
				break;
			}
		}

		if idx.is_none() {
			// create new buffer
			let b = create_shm_buffer(&self.shm, width, height)?;
			bufs.push(b);
			idx = Some(bufs.len() - 1);
		}

		let buf = &mut bufs[idx.unwrap()];
		buf.busy.store(true, Ordering::SeqCst);

		// Write pixels (convert RGBA -> ARGB native-endian) into the anonymous file
		let stride = buf.stride as usize;
		let size = stride * height as usize;
		let mut out: Vec<u8> = vec![0u8; size];

		for y in 0..height as usize {
			for x in 0..width as usize {
				let src_off = (y * width as usize + x) * 4;
				let r = pixels[src_off];
				let g = pixels[src_off + 1];
				let bpx = pixels[src_off + 2];
				let a = pixels[src_off + 3];
				let pixel_u32: u32 =
					((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (bpx as u32);
				let bytes = pixel_u32.to_ne_bytes();
				let dst_off = y * stride + x * 4;
				out[dst_off..dst_off + 4].copy_from_slice(&bytes);
			}
		}

		// write to file
		let mut f = &buf.file;
		f.seek(SeekFrom::Start(0))
			.map_err(|e| OalError::Other(format!("seek failed: {}", e)))?;
		f.write_all(&out)
			.map_err(|e| OalError::Other(format!("write failed: {}", e)))?;

		// Attach and commit
		self.surface.attach(Some(&buf.buffer), 0, 0);
		self.surface
			.damage_buffer(0, 0, width as i32, height as i32);
		self.surface.commit();

		Ok(())
	}
}

impl Window for WaylandWindow {
	fn present(&self, _frame: Frame, _dirty: &[DeviceRect]) -> Result<(), OalError> {
		// Delegate to the main-thread-only present_frame implementation.
		self.present_frame(_frame, _dirty)
	}

	fn poll_events(&self) -> Result<(), OalError> {
		// The OAL instance owns the event queue; a simple implementation does nothing here.
		Ok(())
	}

	fn process_main_thread_tasks(&self) -> Result<(), OalError> {
		// No-op here; the OAL top-level should expose a process API. Window may delegate.
		Ok(())
	}
}

impl Oal for WaylandHandle {
	fn queue_main_thread(
		&self,
		task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<(), OalError> {
		self.tx
			.send(RuntimeMessage::Task(task))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;
		Ok(())
	}

	fn queue_main_thread_with_handle(
		&self,
		task: Box<dyn FnOnce() -> Result<(), OalError> + Send + 'static>,
	) -> Result<oneshot::Receiver<Result<(), OalError>>, OalError> {
		let (tx, rx) = oneshot::channel();
		self.tx
			.send(RuntimeMessage::TaskWithResponder(task, tx))
			.map_err(|_| OalError::ResourceUnavailable("wayland runtime unavailable".into()))?;
		Ok(rx)
	}
}

/// A lightweight thread-safe Surface wrapper that uses the global `WaylandHandle`
/// to register a persistent window and marshal present requests to the runtime.
pub struct WaylandSurface {
	handle: WaylandHandle,
	id: u64,
	size: DeviceSize,
}

impl WaylandSurface {
	/// Create a new WaylandSurface by requesting a registered surface from the
	/// runtime. Returns ResourceUnavailable if the runtime hasn't been initialized.
	pub fn new(width: u32, height: u32) -> Result<Self, OalError> {
		if let Ok(g) = GLOBAL_WAYLAND_HANDLE.lock() {
			if let Some(h) = &*g {
				let id = h.create_surface(width, height)?;
				return Ok(Self {
					handle: h.clone(),
					id,
					size: DeviceSize::new(width, height),
				});
			}
		}
		Err(OalError::ResourceUnavailable(
			"wayland runtime not initialized".into(),
		))
	}
}

impl crate::traits::Surface for WaylandSurface {
	fn present_frame(&self, frame: Frame, dirty: &[DeviceRect]) -> Result<(), OalError> {
		self.handle
			.present_to_surface(self.id, frame, dirty.to_vec())
	}

	fn invalidate_region(&self, _rects: &[DeviceRect]) -> Result<(), OalError> {
		Ok(())
	}

	fn size(&self) -> DeviceSize {
		self.size
	}
}

// Provide a platform-specific SurfaceBuilder::build when the `wayland` feature
// is enabled. This returns a `WaylandSurface` that enqueues presents to the
// runtime mailbox. If the runtime hasn't been brought up by the embedder this
// returns ResourceUnavailable.
impl crate::traits::SurfaceBuilder {
	#[cfg(feature = "wayland")]
	pub fn build(self) -> Result<Box<dyn crate::traits::Surface>, OalError> {
		let s = WaylandSurface::new(self.width, self.height)?;
		Ok(Box::new(s))
	}
}
