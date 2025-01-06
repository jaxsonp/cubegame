
mod render;

use winit::{
	window::Window,
	event_loop::{ControlFlow, EventLoop},
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId};


use render::Renderer;

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

	log::info!("Starting");
	event_loop
		.run_app(&mut application)
		.expect("Event loop error");
}

/// Application handler struct
pub struct Application {
	window_attributes: WindowAttributes,
	renderer: Option<Renderer>,
}
impl Application {
	/// Application constructor
	pub fn new(window_attributes: WindowAttributes) -> Application {
		Self {
			window_attributes,
			renderer: None,
		}
	}

	/// Creates the window and the renderer, needs to be invoked after the first "resumed" event
	fn initialize(&mut self, event_loop: &ActiveEventLoop) {
		let window = event_loop.create_window(self.window_attributes.clone()).expect("Failed to create window");
		self.renderer = Some(Renderer::new(window));
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
			if window_id == renderer.window.id() {
				renderer.handle_event(event, event_loop);
			}
		}
	}

	// When the app has been suspended
	fn suspended(&mut self, _: &ActiveEventLoop) {
		println!("Suspended");
	}

	// event loop is exiting
	fn exiting(&mut self, _: &ActiveEventLoop) {
		log::info!("Exiting");
	}

	// received a memory warning
	fn memory_warning(&mut self, _: &ActiveEventLoop) {}
}
