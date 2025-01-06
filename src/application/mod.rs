
mod framerate;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};

use crate::render::Renderer;
use framerate::FramerateManager;

/// Application handler struct
pub struct Application {
	window_attributes: WindowAttributes,
	renderer: Option<Renderer>,
	pub framerate_manager: FramerateManager,
}
impl Application {
	/// Application constructor
	pub fn new(window_attributes: WindowAttributes) -> Application {
		Self {
			window_attributes,
			renderer: None,
			framerate_manager: FramerateManager::new(),
		}
	}

	/// Creates the window and the renderer, needs to be invoked after the first "resumed" event
	fn initialize(&mut self, event_loop: &ActiveEventLoop) {
		let window = event_loop.create_window(self.window_attributes.clone()).expect("Failed to create window");
		self.renderer = Some(Renderer::new(window));
	}

	pub fn set_max_fps(&mut self, fps: u64) {
		self.framerate_manager.set_max_fps(fps);
		log::info!("Set max FPS to {fps}");
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
		if let Some(renderer) = self.renderer.as_mut() {
			if window_id != renderer.window.id() {
				return;
			}

			match event {
				WindowEvent::RedrawRequested => {
					self.framerate_manager.tick();

					renderer.window.request_redraw();
					match renderer.render() {
						Ok(_) => {}
						Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
							// Reconfigure the surface if it's lost or outdated
							renderer.resize(renderer.size)
						}
						Err(wgpu::SurfaceError::OutOfMemory) => {
							log::error!("OutOfMemory");
							event_loop.exit();
						}
						Err(wgpu::SurfaceError::Timeout) => {
							log::warn!("Surface timeout")
						}
					}
				}
				WindowEvent::CloseRequested => event_loop.exit(),
				WindowEvent::Resized(physical_size) => {
					renderer.resize(physical_size);
				}

				_ => {}
			}
		}
	}

	// When the app has been suspended
	fn suspended(&mut self, _: &ActiveEventLoop) {
		log::debug!("Suspended");
	}

	// event loop is exiting
	fn exiting(&mut self, _: &ActiveEventLoop) {
		log::info!("Exiting");
	}

	// received a memory warning
	fn memory_warning(&mut self, _: &ActiveEventLoop) {}
}
