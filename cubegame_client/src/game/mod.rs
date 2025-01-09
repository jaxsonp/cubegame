mod world;
pub mod player;

use player::Player;
use world::LoadedWorld;

pub struct Game {
	pub player: Player,
	pub world: LoadedWorld,
}
impl Game {
	pub fn new() -> Self {
		Game {
			player: Player::new(),
			world: LoadedWorld::new(),
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);
	}
}
