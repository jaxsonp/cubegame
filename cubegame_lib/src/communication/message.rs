use serde::{Deserialize, Serialize};
use crate::*;


// message content formats
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
	/// Create a world with name
	CreateWorld(WorldGenesisData, String),
	/// Attempt to load a world from its name
	LoadWorld(String),
	/// Change this block at this position in the loaded world
	BlockUpdate(ChunkPos, LocalBlockPos, BlockData),
}

#[derive(Serialize, Deserialize)]
pub enum ServerResponse {
	Ack,
	Err(ErrorMessage),
}

#[derive(Serialize, Deserialize)]
pub enum ErrorMessage {
	NoLoadedWorld,
	WorldDoesNotExist,
}