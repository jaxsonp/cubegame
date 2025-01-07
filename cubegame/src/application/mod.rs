mod framerate;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::{player::Player, render::Renderer};
use framerate::FramerateManager;

/// Application handler struct
pub struct Application {
	renderer: Renderer,
	pub framerate_manager: FramerateManager,
	pub player: Player,
}
impl Application {
	/// Application constructor
	pub fn new(window: Window) -> Application {
		let mut framerate_manager = FramerateManager::new();
		framerate_manager.set_max_fps(60);

		Self {
			renderer: Renderer::new(window),
			framerate_manager,
			player: Player::new(),
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);
		self.renderer.camera.player_pov(&self.player);
	}
}
impl ApplicationHandler for Application {
	fn resumed(&mut self, _: &ActiveEventLoop) {
		log::debug!("Resumed")
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		if window_id != self.renderer.window.id() {
			return;
		}

		self.player.handle_input(&event);

		match event {
			WindowEvent::RedrawRequested => {
				let dt = self.framerate_manager.tick();
				self.update(dt);

				self.renderer.window.request_redraw();
				match self.renderer.render() {
					Ok(_) => {}
					Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
						// Reconfigure the surface if it's lost or outdated
						self.renderer.reconfigure()
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
				self.renderer.resize(physical_size);
			}
			_ => {}
		}
	}

	fn suspended(&mut self, _: &ActiveEventLoop) {
		log::debug!("Suspended");
	}

	// event loop is exiting
	fn exiting(&mut self, _: &ActiveEventLoop) {
		log::info!("Exiting");
	}

	// received a memory warning
	fn memory_warning(&mut self, _: &ActiveEventLoop) {
		log::warn!("Received memory warning");
	}
}

/// Application handler struct, wraps initialization logic around the Application struct
pub enum ApplicationState {
	/// Uninitialized state, with the attributes to initialize the window with
	Uninitialized(WindowAttributes),
	/// Initialized state
	Initialized(Application),
}
impl ApplicationState {
	pub fn new(window_attributes: WindowAttributes) -> ApplicationState {
		ApplicationState::Uninitialized(window_attributes)
	}

	fn initialize(&mut self, event_loop: &ActiveEventLoop) {
		if let ApplicationState::Uninitialized(window_attributes) = self {
			let window = event_loop
				.create_window(window_attributes.clone())
				.expect("Failed to create window");
			*self = ApplicationState::Initialized(Application::new(window));
		} else {
			log::warn!("Tried to double initialize application state");
		}
	}
}
impl ApplicationHandler for ApplicationState {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		match self {
			ApplicationState::Uninitialized(_) => {
				self.initialize(event_loop);
				self.resumed(event_loop);
			}
			ApplicationState::Initialized(application) => application.resumed(event_loop),
		}
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		if let ApplicationState::Initialized(application) = self {
			application.window_event(event_loop, window_id, event);
		}
	}

	// When the app has been suspended
	fn suspended(&mut self, event_loop: &ActiveEventLoop) {
		if let ApplicationState::Initialized(application) = self {
			application.suspended(event_loop);
		} else {
			log::warn!("Tried to suspended uninitialized application");
		}
	}

	// event loop is exiting
	fn exiting(&mut self, event_loop: &ActiveEventLoop) {
		if let ApplicationState::Initialized(application) = self {
			application.exiting(event_loop);
		} else {
			log::warn!("Exiting uninitialized application");
		}
	}

	// received a memory warning
	fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
		if let ApplicationState::Initialized(application) = self {
			application.memory_warning(event_loop)
		}
	}
}
