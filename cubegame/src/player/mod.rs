pub mod controller;

use controller::PlayerController;

pub struct Player {
	controller: PlayerController,
}
impl Player {
	pub fn new() -> Self {
		Self {
			controller: PlayerController::new(),
		}
	}
}