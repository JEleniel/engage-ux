use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

/// Track the currently active GPU device (by pointer identity) so backends
/// and renderers can coordinate cache invalidation and resource recreation
/// when a device is lost and a new one is created.
static CURRENT_DEVICE: OnceLock<Mutex<Option<usize>>> = OnceLock::new();

/// Registry for callbacks invoked when a device is recreated. Callbacks
/// receive (old_device_id, new_device_id) and should perform any necessary
/// reupload or state migration for GPU resources tied to the new device.
// Type aliases to reduce visible type complexity for clippy
// Callback types (single callback) - use aliases so function signatures
// remain readable and avoid triggering `type_complexity` lint.
type DeviceRecreatedCallback = Arc<dyn Fn(usize, usize) + Send + Sync>;
type DeviceReuploadCallback = Arc<dyn Fn(&wgpu::Device, &wgpu::Queue) + Send + Sync>;

type DeviceRecreatedCallbacks = Mutex<HashMap<usize, DeviceRecreatedCallback>>;
static DEVICE_RECREATED_CALLBACKS: OnceLock<DeviceRecreatedCallbacks> = OnceLock::new();
/// Registry for callbacks that perform GPU reuploads when a new device and
/// queue become available. Callbacks receive (&wgpu::Device, &wgpu::Queue).
type DeviceReuploadCallbacks = Mutex<HashMap<usize, DeviceReuploadCallback>>;
static DEVICE_REUPLOAD_CALLBACKS: OnceLock<DeviceReuploadCallbacks> = OnceLock::new();
static NEXT_REUPLOAD_CALLBACK_ID: OnceLock<Mutex<usize>> = OnceLock::new();
static NEXT_CALLBACK_ID: OnceLock<Mutex<usize>> = OnceLock::new();

/// Set the active device identifier. The device id is expected to be the
/// pointer value of the `wgpu::Device` instance (cast to usize) used by
/// the backend. Call this once the device is created or recreated.
pub fn set_active_device(device_id: usize) {
	let lock = CURRENT_DEVICE.get_or_init(|| Mutex::new(None));
	if let Ok(mut g) = lock.lock() {
		*g = Some(device_id);
	}
}

/// Clear the active device record (e.g. during shutdown).
pub fn clear_active_device() {
	if let Some(lock) = CURRENT_DEVICE.get()
		&& let Ok(mut g) = lock.lock() {
			*g = None;
		}
}

/// Return the currently active device id, if any.
pub fn get_active_device() -> Option<usize> {
	if let Some(lock) = CURRENT_DEVICE.get()
		&& let Ok(g) = lock.lock() {
			return *g;
		}
	None
}

/// Register a callback to be invoked when a device is recreated. The
/// callback receives (old_device_id, new_device_id). Returns a handle id
/// that can be used to unregister the callback.
pub fn register_device_recreated_callback(cb: DeviceRecreatedCallback) -> usize {
	let map_lock = DEVICE_RECREATED_CALLBACKS.get_or_init(|| Mutex::new(HashMap::new()));
	let id_lock = NEXT_CALLBACK_ID.get_or_init(|| Mutex::new(1));
	let mut idg = id_lock.lock().unwrap();
	let id = *idg;
	*idg += 1;
	if let Ok(mut map) = map_lock.lock() {
		map.insert(id, cb);
	}
	id
}

/// Unregister a previously registered callback by handle id.
pub fn unregister_device_recreated_callback(handle_id: usize) {
	if let Some(map_lock) = DEVICE_RECREATED_CALLBACKS.get()
		&& let Ok(mut map) = map_lock.lock() {
			map.remove(&handle_id);
		}
}

/// Register a callback that will be invoked when the backend has a new
/// `wgpu::Device` and `wgpu::Queue` available after device recreation.
/// Returns a handle that can be used to unregister the callback.
pub fn register_device_reupload_callback(
	cb: DeviceReuploadCallback,
) -> usize {
	let map_lock = DEVICE_REUPLOAD_CALLBACKS.get_or_init(|| Mutex::new(HashMap::new()));
	let id_lock = NEXT_REUPLOAD_CALLBACK_ID.get_or_init(|| Mutex::new(1));
	let mut idg = id_lock.lock().unwrap();
	let id = *idg;
	*idg += 1;
	if let Ok(mut map) = map_lock.lock() {
		map.insert(id, cb);
	}
	id
}

/// Unregister a previously registered reupload callback.
pub fn unregister_device_reupload_callback(handle_id: usize) {
	if let Some(map_lock) = DEVICE_REUPLOAD_CALLBACKS.get()
		&& let Ok(mut map) = map_lock.lock() {
			map.remove(&handle_id);
		}
}

/// Invoke all registered device reupload callbacks with the provided
/// `device` and `queue`. Callbacks are executed in a best-effort fashion
/// and panics are caught to avoid bringing down the event loop.
pub fn call_device_reupload_callbacks(device: &wgpu::Device, queue: &wgpu::Queue) {
	if let Some(map_lock) = DEVICE_REUPLOAD_CALLBACKS.get()
		&& let Ok(map) = map_lock.lock() {
			for (_id, cb) in map.iter() {
				let cb = cb.clone();
				let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
					(cb)(device, queue)
				}));
			}
		}
}

/// Notify that a device was recreated. This will invalidate GPU caches for
/// the old device, set the new active device id, and invoke registered
/// callbacks to allow subsystems to reupload their GPU resources.
pub fn device_recreated(old_device_id: usize, new_device_id: usize) {
	// Invalidate GPU caches tied to old device.
	crate::oal::renderer::invalidate_gpu_caches_for_device(old_device_id);

	// Set new active device.
	set_active_device(new_device_id);

	// Invoke callbacks (best-effort; ignore panics to avoid bringing down the event loop).
	if let Some(map_lock) = DEVICE_RECREATED_CALLBACKS.get()
		&& let Ok(map) = map_lock.lock() {
			for (_id, cb) in map.iter() {
				let cb = cb.clone();
				// Run callback; catch unwinds to keep event loop stable.
				let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
					(cb)(old_device_id, new_device_id)
				}));
			}
		}
}
