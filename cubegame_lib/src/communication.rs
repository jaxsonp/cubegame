use crate::*;
use serde::{Deserialize, Serialize};

/// Common trait for all cubegame communications, provides uniform methods for encoding and decoding messages into bytes
pub trait Communication<'de>: Serialize + Deserialize<'de> {
	fn encode(&self) -> Vec<u8> {
		rmp_serde::encode::to_vec(&self).unwrap()
	}
	fn decode(data: &'de [u8]) -> Self {
		rmp_serde::decode::from_slice(&data).unwrap()
	}
}

// message content formats
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
	/// Request chunk data at this position
	LoadChunk(ChunkPos),
	/// Change this block at this position in the loaded world
	BlockUpdate(ChunkPos, LocalBlockPos, BlockData),
}
impl Communication<'_> for ServerMessage {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerResponse {
	/// General acknowledgement
	Ack,
	/// Error
	Err(ErrorMessage),
	/// Response to LoadChunk request
	LoadChunkOK(ChunkDeltaData),
}
impl Communication<'_> for ServerResponse {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorMessage {
	NoLoadedWorld,
	WorldDoesNotExist,
}
