mod framerate;

use std::sync::Arc;
use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowAttributes, WindowId},
};

use crate::{game::Game, render::Renderer};
use framerate::FramerateManager;

/// Application handler struct
pub struct Application {
	renderer: Renderer,
	/// Application window (needs to be arced because the renderer and the render surface
	/// constructor (which is async) needs it
	window: Arc<Window>,
	pub framerate_manager: FramerateManager,
	pub game: Option<Game>,
}
impl Application {
	/// Application constructor
	pub fn new(window: Window) -> Result<Application, ()> {
		let window = Arc::new(window);

		let mut framerate_manager = FramerateManager::new();
		framerate_manager.set_max_fps(60);

		let world = Some(Game::new());
		let renderer = match Renderer::new(window.clone()) {
			Ok(renderer) => renderer,
			Err(()) => {
				return Err(());
			}
		};

		Ok(Self {
			window,
			renderer,
			framerate_manager,
			game: world,
		})
	}

	pub fn update(&mut self, dt: f32) {
		self.window
			.set_title(format!("Cubegame ({} fps)", self.framerate_manager.current_fps).as_str());
		if let Some(world) = &mut self.game {
			world.update(dt);
			self.renderer.camera.player_pov(&world.player);
		}
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

		if let Some(world) = &mut self.game {
			world.player.handle_input(&event);
		}

		match event {
			WindowEvent::RedrawRequested => {
				let dt = self.framerate_manager.tick();
				self.update(dt);

				self.window.request_redraw();

				// remeshing world chunks if necessary
				if let Some(game) = &mut self.game {
					if game.world.chunk.needs_remesh {
						game.world.chunk.regenerate_meshes(&self.renderer);
					}
				}
				match self.renderer.render(&self.game) {
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

	fn initialize(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ()> {
		if let ApplicationState::Uninitialized(window_attributes) = self {
			let window = event_loop
				.create_window(window_attributes.clone())
				.expect("Failed to create window");
			match Application::new(window) {
				Ok(app) => {
					*self = ApplicationState::Initialized(app);
					return Ok(());
				}
				Err(()) => {
					log::error!("Error while initializing application");
					event_loop.exit();
					return Err(());
				}
			};
		} else {
			log::warn!("Tried to double initialize application state");
			return Ok(());
		}
	}
}
impl ApplicationHandler for ApplicationState {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		match self {
			ApplicationState::Uninitialized(_) => {
				// try to initialize
				if self.initialize(event_loop).is_ok() {
					// "resend" resume event if initialization was successful
					self.resumed(event_loop);
				}
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
		}
	}

	// received a memory warning
	fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
		if let ApplicationState::Initialized(application) = self {
			application.memory_warning(event_loop)
		}
	}
}
