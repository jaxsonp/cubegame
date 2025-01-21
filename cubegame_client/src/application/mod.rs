mod framerate;

use crate::{game::Game, render::Renderer, INTEGRATED_SERVER_PORT};
use framerate::FramerateManager;
use std::sync::Arc;
use winit::event::{DeviceEvent, DeviceId};
use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::ActiveEventLoop,
	window::{Window, WindowAttributes, WindowId},
};

/// Application handler struct
pub enum ApplicationState {
	InGame {
		renderer: Renderer,
		/// Application window (needs to be arced because the renderer and the render surface
		/// constructor (which is async) needs it
		window: Arc<Window>,
		game: Game,
		framerate_manager: FramerateManager,
	},
	/// Uninitialized state, with the attributes to initialize the window with
	Uninitialized(WindowAttributes),
}
impl ApplicationState {
	pub fn new(window_attributes: WindowAttributes) -> ApplicationState {
		ApplicationState::Uninitialized(window_attributes)
	}

	fn initialize(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ()> {
		if let ApplicationState::Uninitialized(window_attributes) = self {
			// Creating application and all its state and stuff
			let window = Arc::new(
				event_loop
					.create_window(window_attributes.clone())
					.expect("Failed to create window"),
			);

			let renderer = match Renderer::new(window.clone()) {
				Ok(renderer) => renderer,
				Err(()) => {
					return Err(());
				}
			};
			log::debug!("Instantiated renderer");

			let mut framerate_manager = FramerateManager::new();
			framerate_manager.set_max_fps(60);

			// TODO support connecting to external servers
			// connecting to game server
			let game_server_uri = http::Uri::builder()
				.scheme("ws")
				.authority(format!("localhost:{}", INTEGRATED_SERVER_PORT))
				.path_and_query("/")
				.build()
				.unwrap();
			let game = Game::new(game_server_uri, window.clone())?;

			*self = ApplicationState::InGame {
				renderer,
				window,
				framerate_manager,
				game,
			};
		} else {
			log::warn!("Tried to double initialize application state");
		}
		return Ok(());
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
			_ => {}
		}
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		window_id: WindowId,
		event: WindowEvent,
	) {
		match self {
			ApplicationState::InGame {
				renderer,
				window,
				framerate_manager,
				game,
			} => {
				game.handle_window_event(&event);

				match event {
					WindowEvent::RedrawRequested => {
						if window_id != window.id() {
							return;
						}

						let dt = framerate_manager.tick();
						game.update(dt);

						window.set_title(
							format!("Cubegame ({} fps)", framerate_manager.current_fps).as_str(),
						);
						window.request_redraw();

						game.prep_meshes(renderer);
						match renderer.render_game(game) {
							Ok(_) => {}
							Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
								// Reconfigure the surface if it's lost or outdated
								renderer.reconfigure_surface()
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
					WindowEvent::Resized(physical_size) => renderer.resize(physical_size),
					_ => {}
				}
			}
			ApplicationState::Uninitialized(_) => match event {
				WindowEvent::CloseRequested => event_loop.exit(),
				_ => {}
			},
		}
	}

	fn device_event(
		&mut self,
		_event_loop: &ActiveEventLoop,
		_device_id: DeviceId,
		event: DeviceEvent,
	) {
		match self {
			ApplicationState::InGame { game, .. } => {
				game.handle_device_event(&event);
			}
			ApplicationState::Uninitialized(_) => {}
		}
	}

	// When the app has been suspended
	fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
		log::warn!("Application suspended");
	}

	// event loop is exiting
	fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
		log::info!("Exiting");

		match self {
			ApplicationState::InGame { game, .. } => {
				game.shutdown();
			}
			_ => {}
		}
	}

	// received a memory warning
	fn memory_warning(&mut self, _event_loop: &ActiveEventLoop) {
		log::warn!("Received memory warning");
	}
}
