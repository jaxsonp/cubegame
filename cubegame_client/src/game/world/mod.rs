use cubegame_lib::{ChunkDeltaData, ChunkPos};

use chunk::LoadedChunk;

mod chunk;

pub struct LoadedWorld {
	pub chunk: LoadedChunk,
}
impl LoadedWorld {
	pub fn new() -> Self {
		LoadedWorld {
			chunk: LoadedChunk::load_from_delta(ChunkDeltaData::empty(ChunkPos(0, 0))),
		}
	}

	pub fn update(&mut self, dt: f32) {}
}
