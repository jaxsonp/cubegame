use winit::event::ElementState;
use winit::{
	event::WindowEvent,
	keyboard::{KeyCode, PhysicalKey},
};

/// Stores state about player input
#[derive(Debug, Default)]
pub struct PlayerController {
	input_forward: bool,
	input_backward: bool,
	input_left: bool,
	input_right: bool,
	input_up: bool,
	input_down: bool,
	input_looking_up: bool,
	input_looking_down: bool,
	input_looking_left: bool,
	input_looking_right: bool,
}
impl PlayerController {
	pub fn new() -> PlayerController {
		Self::default()
	}

	pub fn handle_input(&mut self, window_event: &WindowEvent) {
		match window_event {
			WindowEvent::KeyboardInput {
				device_id: _,
				event,
				is_synthetic: _,
			} => {
				if event.repeat {
					return;
				}

				let pressed = event.state == ElementState::Pressed;

				use KeyCode::*;
				match event.physical_key {
					PhysicalKey::Code(KeyW) => self.input_forward = pressed,
					PhysicalKey::Code(KeyS) => self.input_backward = pressed,
					PhysicalKey::Code(KeyA) => self.input_left = pressed,
					PhysicalKey::Code(KeyD) => self.input_right = pressed,
					PhysicalKey::Code(Space) => self.input_up = pressed,
					PhysicalKey::Code(ShiftLeft) => self.input_down = pressed,
					PhysicalKey::Code(ArrowLeft) => self.input_looking_left = pressed,
					PhysicalKey::Code(ArrowRight) => self.input_looking_right = pressed,
					PhysicalKey::Code(ArrowUp) => self.input_looking_up = pressed,
					PhysicalKey::Code(ArrowDown) => self.input_looking_down = pressed,
					_ => {}
				}
			}
			_ => {}
		}
	}

	pub fn forward(&self) -> bool {
		self.input_forward && !self.input_backward
	}
	pub fn backward(&self) -> bool {
		self.input_backward && !self.input_forward
	}
	pub fn left(&self) -> bool {
		self.input_left && !self.input_right
	}
	pub fn right(&self) -> bool {
		self.input_right && !self.input_left
	}
	pub fn up(&self) -> bool {
		self.input_up && !self.input_down
	}
	pub fn down(&self) -> bool {
		self.input_down && !self.input_up
	}
	pub fn looking_up(&self) -> bool {
		self.input_looking_up && !self.input_looking_down
	}
	pub fn looking_down(&self) -> bool {
		self.input_looking_down && !self.input_looking_up
	}
	pub fn looking_left(&self) -> bool {
		self.input_looking_left && !self.input_looking_right
	}
	pub fn looking_right(&self) -> bool {
		self.input_looking_right && !self.input_looking_left
	}
}
