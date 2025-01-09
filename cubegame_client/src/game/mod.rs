mod chunk;

use cubegame_lib::worldgen;

use crate::player::Player;
use chunk::LoadedChunk;

pub struct LoadedWorld {
	pub player: Player,
	pub chunk: LoadedChunk,
}
impl LoadedWorld {
	pub fn new() -> Self {
		LoadedWorld {
			player: Player::new(),
			chunk: LoadedChunk::new(worldgen::generate_chunk(), 0, 0),
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);
	}
}
