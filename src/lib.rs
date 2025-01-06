
mod render;
mod application;

use winit::{
	window::Window,
	event_loop::{ControlFlow, EventLoop},
};

use application::Application;

pub fn run() {
	env_logger::init();

	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);

	// configuring window
	let window_attributes = Window::default_attributes()
		.with_resizable(true)
		.with_title("Rust graphics test")
		.with_active(true);

	let mut application = Application::new(window_attributes);
	application.set_max_fps(60);

	log::info!("Starting");
	event_loop
		.run_app(&mut application)
		.expect("Event loop error");
}

