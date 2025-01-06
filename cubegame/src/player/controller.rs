#[derive(Debug, Default)]
pub struct PlayerController {
	moving_forward: bool,
	moving_backward: bool,
	moving_left: bool,
	moving_right: bool,
	moving_down: bool,
	moving_up: bool,
	looking_right: bool,
	looking_left: bool,
}
impl PlayerController {
	pub fn new() -> PlayerController {
		Self::default()
	}

	pub fn handle_input(&mut self) {}
}