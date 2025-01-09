mod chunk;

use cubegame_lib::{ChunkDeltaData, ChunkPos};

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
			chunk: LoadedChunk::load_from_delta(ChunkDeltaData::empty(ChunkPos(0, 0))),
		}
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);
	}
}
