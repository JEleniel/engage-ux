use crate::errors::Result;
use crate::oal::{Canvas, DeviceMetrics, Unit};

/// Renderer trait - minimal contract for OAL renderers.
pub trait Renderer: Send {
	/// Render the given canvas and view into the backing surface.
	fn render(&mut self, canvas: &Canvas, metrics: &DeviceMetrics, unit: Unit) -> Result<()>;

	/// Optional GPU render submission hook.
	///
	/// Implementors may return a boxed `RenderCallback` which will be invoked
	/// on the backend's GPU/context-owning thread with a backend-specific
	/// context provided as `&mut dyn Any` (for example a `WgpuRenderContext`).
	/// The default implementation returns `None`, indicating no GPU submission
	/// is available and the OAL should fall back to software rendering via
	/// `render`.
	fn try_gpu_render(
		&mut self,
		canvas: &Canvas,
		metrics: &DeviceMetrics,
		unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		let _ = (canvas, metrics, unit);
		None
	}
}

/// A noop software renderer for tests and headless operation.
pub struct NoopRenderer;

impl NoopRenderer {
	/// Create a new `NoopRenderer` instance. Provided for tests and
	/// headless operation where rendering is intentionally a no-op.
	pub fn new() -> Self {
		Self
	}
}

impl Renderer for NoopRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Intentionally no-op.
		Ok(())
	}
}

/// Choose the best available renderer: placeholder; keep simple for now.
#[allow(dead_code)]
pub fn choose_renderer() -> Box<dyn Renderer> {
	Box::new(NoopRenderer::new())
}

/// A simple Skia-based software renderer fallback.
#[cfg(feature = "skia")]
pub struct SkiaSoftwareRenderer {
	// Placeholder for renderer configuration (fonts, caching, etc.)
}

#[cfg(feature = "skia")]
impl SkiaSoftwareRenderer {
	pub fn new() -> Self {
		Self {}
	}
}

#[cfg(feature = "skia")]
impl Renderer for SkiaSoftwareRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Simple software path using skia-safe would go here. For now
		// this is a placeholder that indicates software rendering occurred.
		Ok(())
	}

	// No GPU submission by default for the software renderer.
}

// A minimal wgpu-backed renderer that returns a GPU job which records a
// simple render pass into the provided `WgpuRenderContext`. This renderer
// is available only when the native winit backend is enabled since it
// requires the `WgpuRenderContext` type.
#[cfg(feature = "native-winit")]
pub struct WgpuRenderer {
	// Simple color to clear/draw with; in a real renderer this would be
	// replaced by pipeline/shader state and resources.
	pub color: wgpu::Color,
}

#[cfg(feature = "native-winit")]
impl WgpuRenderer {
	pub fn new(color: wgpu::Color) -> Self {
		Self { color }
	}
}

#[cfg(feature = "native-winit")]
impl Renderer for WgpuRenderer {
	fn render(&mut self, _canvas: &Canvas, _metrics: &DeviceMetrics, _unit: Unit) -> Result<()> {
		// Fallback synchronous path is a no-op for this minimal renderer.
		Ok(())
	}

	fn try_gpu_render(
		&mut self,
		_canvas: &Canvas,
		_metrics: &DeviceMetrics,
		_unit: Unit,
	) -> Option<Box<dyn crate::oal::backend::RenderCallback>> {
		// The minimal WgpuRenderer will not provide a GPU job that draws
		// recorded `Canvas` primitives here because we need to capture the
		// canvas command list at the time the renderer is asked. Instead,
		// we return a closure that expects the backend to invoke it during
		// Present with a `WgpuRenderContext` available. The closure will
		// create a simple instanced quad pipeline and draw rectangle-like
		// primitives; circles/rounded rects will be rendered as SDF in the
		// fragment shader when possible.
		let prims = _canvas.drain_commands();

		// Define a typed instance payload for GPU uploads. Using a
		// #[repr(C)] struct with bytemuck makes the upload safe and clear
		// without unsafe code.
		use bytemuck::{Pod, Zeroable};

		#[repr(C)]
		#[derive(Copy, Clone, Pod, Zeroable)]
		struct Instance {
			center: [f32; 2],
			size: [f32; 2],
			color: [f32; 4],
			shape: f32,
			radii: [f32; 2],
			stroke: f32,
			// padding to 12 floats isn't required because fields total 12 f32s
		}

		Some(Box::new(
			move |backend_ctx: &mut dyn std::any::Any| -> Result<()> {
				use crate::errors::OalError;
				use crate::oal::canvas::Primitive;
				use crate::oal::winit_adapter::WgpuRenderContext;
				use wgpu::util::DeviceExt;

				// We expect the backend to supply a WgpuRenderContext with
				// a valid encoder and an acquired view during Present.
				if let Some(ctx) = backend_ctx.downcast_mut::<WgpuRenderContext>() {
					if let (Some(view), Some(encoder)) = (ctx.view, ctx.encoder.as_mut()) {
						let device = &ctx.device;
						let queue = &ctx.queue;
						let config = &ctx.config;
						let surface_w = config.width as f32;
						let surface_h = config.height as f32;

						// For this minimal pipeline we create a small unit-quad
						// vertex buffer and an index buffer.
						let vertex_data: [f32; 8] = [-0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5];
						let vertex_buf =
							device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
								label: Some("unit-quad-verts"),
								contents: bytemuck::cast_slice(&vertex_data),
								usage: wgpu::BufferUsages::VERTEX,
							});

						let index_data: [u16; 6] = [0, 1, 2, 2, 1, 3];
						let index_buf =
							device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
								label: Some("unit-quad-indices"),
								contents: bytemuck::cast_slice(&index_data),
								usage: wgpu::BufferUsages::INDEX,
							});

						// Gather primitives from the canvas snapshot captured earlier
						// by the renderer call site. We expect the renderer to have
						// drained commands into the job's closure capture; if none
						// exist, we simply perform a clear and return.
						// NOTE: The closure is invoked during Present; callers should
						// arrange to capture commands when creating the job.

						// Build a WGSL shader that renders rectangles, circles and
						// rounded rects using SDFs with AA. Instances now include:
						// center.xy, size.xy, color.rgba, shape_flag, radii.xy, stroke_width.
						let shader_src = r#"
							struct VertexOut {
								@builtin(position) position: vec4<f32>;
								@location(0) color: vec4<f32>;
								@location(1) local: vec2<f32>;
								@location(2) half_size: vec2<f32>;
								@location(3) shape: f32;
								@location(4) radii: vec2<f32>;
								@location(5) stroke: f32;
							};

							@vertex
							fn vs(
								@location(0) pos: vec2<f32>,
								@location(1) offset: vec2<f32>,
								@location(2) half_size: vec2<f32>,
								@location(3) color: vec4<f32>,
								@location(4) shape: f32,
								@location(5) radii: vec2<f32>,
								@location(6) stroke: f32
							) -> VertexOut {
								var out: VertexOut;
								let local = pos * half_size;
								let p = offset + local;
								out.position = vec4<f32>(p.x, p.y, 0.0, 1.0);
								out.color = color;
								out.local = local;
								out.half_size = half_size;
								out.shape = shape;
								out.radii = radii;
								out.stroke = stroke;
								return out;
							}

							fn rect_sdf(p: vec2<f32>, half_ext: vec2<f32>) -> f32 {
								let q = abs(p) - half_ext;
								return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0);
							}

							fn rounded_rect_sdf(p: vec2<f32>, half_ext: vec2<f32>, rad: vec2<f32>) -> f32 {
								// Conservative approach: use min radius for SDF offset
								let r = vec2<f32>(min(rad.x, rad.y), min(rad.x, rad.y));
								let q = abs(p) - (half_ext - r);
								return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - r.x;
							}

							@fragment
							fn fs(in: VertexOut) -> @location(0) vec4<f32> {
								let half_ext = in.half_size * 0.5;
								var dist: f32;
								if (in.shape < 0.5) {
									// rectangle
									dist = rect_sdf(in.local, half_ext);
								} else if (in.shape < 1.5) {
									// circle
									let r = min(half_ext.x, half_ext.y);
									dist = length(in.local) - r;
								} else {
									// rounded rect
									dist = rounded_rect_sdf(in.local, half_ext, in.radii);
								}

								let aa = fwidth(dist);
								var alpha: f32;
								if (in.stroke > 0.0) {
									let half_stroke = in.stroke * 0.5;
									let d = abs(dist) - half_stroke;
									alpha = if (aa > 0.0) { smoothstep(0.5 * aa, -0.5 * aa, d) } else { if (d < 0.0) { 1.0 } else { 0.0 } };
								} else {
									alpha = if (aa > 0.0) { smoothstep(0.5 * aa, -0.5 * aa, dist) } else { if (dist < 0.0) { 1.0 } else { 0.0 } };
								}

								return vec4<f32>(in.color.rgb, in.color.a * alpha);
							}
						"#;

						let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
							label: Some("gpu-prims-shader"),
							source: wgpu::ShaderSource::Wgsl(shader_src.into()),
						});

						// Pipeline layout and pipeline
						let pipeline_layout =
							device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
								label: Some("prims-pipeline-layout"),
								bind_group_layouts: &[],
								push_constant_ranges: &[],
							});

						let pipeline =
							device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
								label: Some("prims-pipeline"),
								layout: Some(&pipeline_layout),
								vertex: wgpu::VertexState {
									module: &shader,
									entry_point: "vs",
									buffers: &[
										wgpu::VertexBufferLayout {
											array_stride: std::mem::size_of::<[f32; 2]>()
												as wgpu::BufferAddress,
											step_mode: wgpu::VertexStepMode::Vertex,
											attributes: &[wgpu::VertexAttribute {
												offset: 0,
												shader_location: 0,
												format: wgpu::VertexFormat::Float32x2,
											}],
										},
										// Instance buffer: offset(vec2) + half_size(vec2) + color(vec4) + shape(float)
										wgpu::VertexBufferLayout {
											array_stride: std::mem::size_of::<Instance>()
												as wgpu::BufferAddress,
											step_mode: wgpu::VertexStepMode::Instance,
											attributes: &[
												wgpu::VertexAttribute {
													offset: 0,
													shader_location: 1,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 8,
													shader_location: 2,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 16,
													shader_location: 3,
													format: wgpu::VertexFormat::Float32x4,
												},
												wgpu::VertexAttribute {
													offset: 32,
													shader_location: 4,
													format: wgpu::VertexFormat::Float32,
												},
												wgpu::VertexAttribute {
													offset: 36,
													shader_location: 5,
													format: wgpu::VertexFormat::Float32x2,
												},
												wgpu::VertexAttribute {
													offset: 44,
													shader_location: 6,
													format: wgpu::VertexFormat::Float32,
												},
											],
										},
									],
								},
								fragment: Some(wgpu::FragmentState {
									module: &shader,
									entry_point: "fs",
									targets: &[Some(wgpu::ColorTargetState {
										format: config.format,
										blend: Some(wgpu::BlendState::ALPHA_BLENDING),
										write_mask: wgpu::ColorWrites::ALL,
									})],
								}),
								primitive: wgpu::PrimitiveState::default(),
								depth_stencil: None,
								multisample: wgpu::MultisampleState::default(),
								multiview: None,
							});

						// Build instance data from the captured canvas primitives.
						// We use the typed `Instance` struct defined above and
						// push Instance values into a Vec<Instance>, then upload
						// it directly with bytemuck::cast_slice.
						let mut instances: Vec<Instance> = Vec::new();
						for p in prims.iter() {
							match p {
								Primitive::Rect {
									origin,
									size,
									color,
									..
								} => {
									let center_x = origin.0 + size.0 * 0.5;
									let center_y = origin.1 + size.1 * 0.5;
									// Convert to NDC [-1,1]
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = size.0 / surface_w;
									let size_y = size.1 / surface_h;
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											color.red as f32 / 255.0,
											color.green as f32 / 255.0,
											color.blue as f32 / 255.0,
											color.alpha as f32 / 255.0,
										],
										shape: 0.0,
										radii: [0.0, 0.0],
										stroke: 0.0,
									});
								}
								Primitive::Circle {
									center,
									radius,
									color,
									..
								} => {
									let center_x = center.0;
									let center_y = center.1;
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = (radius * 2.0) / surface_w;
									let size_y = (radius * 2.0) / surface_h;
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											color.red as f32 / 255.0,
											color.green as f32 / 255.0,
											color.blue as f32 / 255.0,
											color.alpha as f32 / 255.0,
										],
										shape: 1.0,
										radii: [0.0, 0.0],
										stroke: 0.0,
									});
								}
								Primitive::RoundedRect {
									origin,
									size,
									radii,
									color,
									stroke,
									..
								} => {
									let center_x = origin.0 + size.0 * 0.5;
									let center_y = origin.1 + size.1 * 0.5;
									let ndc_x = center_x / surface_w * 2.0 - 1.0;
									let ndc_y = 1.0 - (center_y / surface_h * 2.0);
									let size_x = size.0 / surface_w;
									let size_y = size.1 / surface_h;
									instances.push(Instance {
										center: [ndc_x, ndc_y],
										size: [size_x, size_y],
										color: [
											color.red as f32 / 255.0,
											color.green as f32 / 255.0,
											color.blue as f32 / 255.0,
											color.alpha as f32 / 255.0,
										],
										shape: 2.0,
										radii: [radii.0 / surface_w, radii.1 / surface_h],
										stroke: if *stroke > 0.0 {
											*stroke / surface_w
										} else {
											0.0
										},
									});
								}
								_ => {
									// Other primitive types not yet supported in GPU
									// path; they will be ignored for now.
								}
							}
						}

						let instance_count = instances.len() as u32;
						let instance_buf = if instance_count > 0 {
							Some(
								device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
									label: Some("prims-instances"),
									contents: bytemuck::cast_slice(&instances),
									usage: wgpu::BufferUsages::VERTEX,
								}),
							)
						} else {
							None
						};

						// Begin render pass
						let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
							label: Some("prims-pass"),
							color_attachments: &[Some(wgpu::RenderPassColorAttachment {
								view,
								resolve_target: None,
								ops: wgpu::Operations {
									load: wgpu::LoadOp::Load,
									store: true,
								},
							})],
							depth_stencil_attachment: None,
						});

						rpass.set_pipeline(&pipeline);
						rpass.set_vertex_buffer(0, vertex_buf.slice(..));
						if let Some(buf) = instance_buf.as_ref() {
							rpass.set_vertex_buffer(1, buf.slice(..));
						}
						rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);

						// Issue an instanced draw using the built instance buffer
						// when we have instances; otherwise draw a single unit
						// quad to keep the frame valid.
						if instance_count > 0 {
							rpass.draw_indexed(0..6, 0, 0..instance_count);
						} else {
							rpass.draw_indexed(0..6, 0, 0..1);
						}

						drop(rpass);

						Ok(())
					} else {
						Err(OalError::Renderer(
							"missing view/encoder in WgpuRenderContext".into(),
						))
					}
				} else {
					Err(OalError::Renderer(
						"invalid backend context for WgpuRenderer".into(),
					))
				}
			},
		))
	}
}

// Public docs: A Renderer is a minimal drawing backend used by the OAL to
// present the logical `Canvas` contents into a platform surface. The
// `NoopRenderer` is provided for tests and headless operation; real
// renderers should implement `render` and handle the supplied `DeviceMetrics`
// and `Unit` to produce pixels.
