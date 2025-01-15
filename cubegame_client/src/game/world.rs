use std::collections::HashMap;

use cubegame_lib::ChunkPos;

use crate::game::chunk::LoadedChunk;
use crate::game::player::Player;

/// Data about the loaded world
pub struct WorldData {
	/// client's player
	pub player: Player,
	/// Loaded chunks
	pub chunks: HashMap<ChunkPos, LoadedChunk>,
}
impl WorldData {
	pub fn new() -> Self {
		WorldData {
			player: Player::new(),
			chunks: HashMap::new(),
		}
	}
}
