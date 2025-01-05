#![allow(unused)]

use std::sync::Arc;
use winit::{
	application::ApplicationHandler,
	dpi::LogicalSize,
	event::*,
	event_loop::{ActiveEventLoop, EventLoop},
	keyboard::{KeyCode, PhysicalKey},
	window::{Window, WindowAttributes, WindowId},
};
use pollster::FutureExt as _;
use winit::event_loop::ControlFlow;

pub struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	window: Arc<Window>,
}

impl Renderer {
	pub fn new(window: Arc<Window>) -> Renderer {

		return async move {
			let size = window.inner_size();

			let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
				backends: wgpu::Backends::PRIMARY,
				..Default::default()
			});

			// drawing target texture
			let surface = instance.create_surface(window.clone()).unwrap();

			// handle to GPU
			let adapter = instance
				.request_adapter(&wgpu::RequestAdapterOptions {
					power_preference: wgpu::PowerPreference::default(),
					compatible_surface: Some(&surface),
					force_fallback_adapter: false,
				})
				.await
				.unwrap();

			let (device, queue) = adapter
				.request_device(
					&wgpu::DeviceDescriptor {
						required_features: wgpu::Features::empty(),
						required_limits: wgpu::Limits::default(),
						label: None,
						// performance over efficiency for memory management
						memory_hints: wgpu::MemoryHints::Performance,
					},
					None,
				)
				.await
				.unwrap();
			let surface_capabilities = surface.get_capabilities(&adapter);

			// we doing sRGB
			let surface_format = surface_capabilities
				.formats
				.iter()
				.find(|f| f.is_srgb())
				.copied()
				.unwrap_or(surface_capabilities.formats[0]);
			let config = wgpu::SurfaceConfiguration {
				usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
				format: surface_format,
				width: size.width,
				height: size.height,
				present_mode: surface_capabilities.present_modes[0],
				alpha_mode: surface_capabilities.alpha_modes[0],
				view_formats: vec![],
				desired_maximum_frame_latency: 2,
			};

			println!("Instantiated renderer");
			Self {
				surface,
				device,
				queue,
				config,
				size,
				window,
			}
		}.block_on()
	}

	pub fn handle_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop) {
		match event {
			WindowEvent::RedrawRequested => {
				// This tells winit that we want another frame after this one
				self.window.request_redraw();

				/*if !surface_configured {
					return;
				}*/

				self.update();
				match self.render() {
					Ok(_) => {}
					// Reconfigure the surface if it's lost or outdated
					Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
						self.resize(self.size)
					}
					// The system is out of memory, we should probably quit
					Err(wgpu::SurfaceError::OutOfMemory) => {
						log::error!("OutOfMemory");
						event_loop.exit();
					}

					// This happens when the a frame takes too long to present
					Err(wgpu::SurfaceError::Timeout) => {
						log::warn!("Surface timeout")
					}
				}
			}
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(physical_size) => {
				self.resize(physical_size);
			}
			_ => {}
		}
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	// processes an event?
	pub fn input(&mut self, event: &WindowEvent) -> bool {
		return false; // TODO update this when we want to capture events
	}

	fn update(&mut self) {
		//todo!()
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		// for talking w GPU
		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 0.1,
						g: 0.2,
						b: 0.3,
						a: 1.0,
					}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});
		drop(render_pass);

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

/// Application handler struct
pub struct Application {
	window: Option<Arc<Window>>,
	renderer: Option<Renderer>,
}
impl Application {
	/// Application constructor
	pub fn uninitialized() -> Application {
		Self {
			window: None,
			renderer: None,
		}
	}

	/// Creates the window and the renderer, needs to be invoked after the first "resumed" event
	fn initialize(&mut self, event_loop: &ActiveEventLoop) {
		let window_attributes = Window::default_attributes()
			.with_resizable(true)
			.with_title("Rust graphics test")
			.with_active(true);

		let window = Arc::new(event_loop.create_window(window_attributes).expect("Failed to create window"));
		self.renderer = Some(Renderer::new(Arc::clone(&window)));
	}
}
impl ApplicationHandler for Application {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		if self.renderer.is_none() {
			self.initialize(event_loop)
		}
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		//println!("Window Event: {:?}", event);
		if let Some(renderer) = self.renderer.as_mut() {
			if window_id == renderer.window.id() {
				renderer.handle_event(event, event_loop);
			}
		}
	}

	// When the app has been suspended
	fn suspended(&mut self, event_loop: &ActiveEventLoop) {
		println!("Suspended");
	}

	// event loop is exiting
	fn exiting(&mut self, event_loop: &ActiveEventLoop) {
		println!("Exiting");
	}

	// received a memory warning
	fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {}

}
