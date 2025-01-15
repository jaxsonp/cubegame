mod application;
pub mod game;
pub mod render;

use winit::{
	event_loop::{ControlFlow, EventLoop},
	window::Window,
};

use application::ApplicationState;

pub const INTEGRATED_SERVER_PORT: u16 = 5005;

pub fn run_client() {
	env_logger::init();

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);

	// configuring window
	let window_attributes = Window::default_attributes()
		.with_resizable(true)
		.with_title("Cubegame")
		.with_active(true);

	let mut app = ApplicationState::new(window_attributes);

	// spawning integrated server
	match std::thread::Builder::new()
		.name("integrated_server".to_string())
		.spawn(|| {
			if cubegame_server::run_server(INTEGRATED_SERVER_PORT).is_err() {
				log::error!("Integrated server exited with failure");
			}
		}) {
		Ok(_) => {}
		Err(_) => {
			log::error!("Failed to start integrated server thread");
			return;
		}
	}

	log::info!("Starting");
	event_loop.run_app(&mut app).expect("Event loop error");
}
