use winit::{
	event::{DeviceEvent, ElementState},
	keyboard::{KeyCode, PhysicalKey},
};

/// Stores state about player input
#[derive(Debug, Default)]
pub struct PlayerController {
	forward_pressed: bool,
	backward_pressed: bool,
	left_pressed: bool,
	right_pressed: bool,
	up_pressed: bool,
	down_pressed: bool,
	turn_amount_x: f64,
	turn_amount_y: f64,
}
impl PlayerController {
	pub fn new() -> PlayerController {
		Self::default()
	}

	pub fn reset(&mut self) {
		self.forward_pressed = false;
		self.backward_pressed = false;
		self.left_pressed = false;
		self.right_pressed = false;
		self.up_pressed = false;
		self.down_pressed = false;
		self.turn_amount_x = 0.0;
		self.turn_amount_y = 0.0;
	}

	pub fn handle_input(&mut self, event: &DeviceEvent) {
		match event {
			DeviceEvent::Key(key_event) => {
				let pressed = key_event.state == ElementState::Pressed;

				use KeyCode::*;
				match key_event.physical_key {
					PhysicalKey::Code(KeyW) => self.forward_pressed = pressed,
					PhysicalKey::Code(KeyS) => self.backward_pressed = pressed,
					PhysicalKey::Code(KeyA) => self.left_pressed = pressed,
					PhysicalKey::Code(KeyD) => self.right_pressed = pressed,
					PhysicalKey::Code(Space) => self.up_pressed = pressed,
					PhysicalKey::Code(ShiftLeft) => self.down_pressed = pressed,
					_ => {}
				}
			}
			DeviceEvent::MouseMotion { delta: (x, y) } => {
				self.turn_amount_x -= *x;
				self.turn_amount_y -= *y;
			}
			_ => {}
		}
	}

	pub fn inputting_forward(&self) -> bool {
		self.forward_pressed && !self.backward_pressed
	}
	pub fn inputting_backward(&self) -> bool {
		self.backward_pressed && !self.forward_pressed
	}
	pub fn inputting_left(&self) -> bool {
		self.left_pressed && !self.right_pressed
	}
	pub fn inputting_right(&self) -> bool {
		self.right_pressed && !self.left_pressed
	}
	pub fn inputting_up(&self) -> bool {
		self.up_pressed && !self.down_pressed
	}
	pub fn inputting_down(&self) -> bool {
		self.down_pressed && !self.up_pressed
	}
	pub fn turn_amount_x(&self) -> f64 {
		self.turn_amount_x
	}
	pub fn turn_amount_y(&self) -> f64 {
		self.turn_amount_y
	}
	pub fn reset_turn_amount(&mut self) {
		self.turn_amount_x = 0.0;
		self.turn_amount_y = 0.0;
	}
	/*pub fn looking_up(&self) -> bool {
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
	}*/
}
