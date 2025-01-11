pub mod blocks;
pub mod worldgen;
pub mod communication;

use bitmask_enum::bitmask;
use serde::{Deserialize, Serialize};
use crate::blocks::AIR_BLOCK_ID;

// constants
pub const WORLD_HEIGHT: usize = 256;
pub const CHUNK_WIDTH: usize = 16;
pub const BLOCKS_PER_CHUNK: usize = CHUNK_WIDTH * CHUNK_WIDTH * WORLD_HEIGHT;

// types
pub type BlockTypeID = u8;

#[bitmask(u8)]
#[bitmask_config(flags_iter)]
pub enum Direction {
	PosX,
	NegX,
	PosY,
	NegY,
	PosZ,
	NegZ,
}

/// Chunk indexing position (x and z coordinates)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChunkPos(pub i32, pub i32);
impl ChunkPos {
	pub fn x(&self) -> i32 {
		self.0
	}
	pub fn z(&self) -> i32 {
		self.1
	}
}
#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash, Serialize, Deserialize)]
/// Local block position within a chunk
pub struct LocalBlockPos {
	/// Both the X and Z pos (most significant 4 bits are x, least significant 4 bits are z)
	xz: u8,
	/// Y position
	y: u8,
}
impl LocalBlockPos {
	pub fn new(x: u8, y: u8, z: u8) -> LocalBlockPos {
		LocalBlockPos {
			xz: ((x & 0b1111) << 4) + (z & 0b1111),
			y,
		}
	}
	/// Creates local block position from index in a chunk's data array
	pub fn from_index(i: usize) -> LocalBlockPos {
		LocalBlockPos::new(
			(i % CHUNK_WIDTH) as u8,
			(i / (CHUNK_WIDTH * CHUNK_WIDTH)) as u8,
			((i / CHUNK_WIDTH) % CHUNK_WIDTH) as u8,
		)
	}
	/// Gets index in chunk data array from local block position
	pub fn to_index(&self) -> usize {
		(self.y() as usize) * CHUNK_WIDTH * CHUNK_WIDTH
			+ (self.z() as usize) * CHUNK_WIDTH
			+ (self.x() as usize)
	}
	/// Returns the position of the local block position adjacent in a certain direction
	pub fn get_neighbor(&self, dir: Direction) -> Option<LocalBlockPos> {
		match dir {
			Direction::PosX => {
				if (self.x() as usize) < (CHUNK_WIDTH - 1) {
					return Some(LocalBlockPos::new(self.x() + 1, self.y(), self.z()));
				}
			}
			Direction::NegX => {
				if self.x() > 0 {
					return Some(LocalBlockPos::new(self.x() - 1, self.y(), self.z()));
				}
			}
			Direction::PosY => {
				if (self.y() as usize) < (WORLD_HEIGHT - 1) {
					return Some(LocalBlockPos {
						xz: self.xz,
						y: self.y + 1,
					});
				}
			}
			Direction::NegY => {
				if self.y > 0 {
					return Some(LocalBlockPos {
						xz: self.xz,
						y: self.y - 1,
					});
				}
			}
			Direction::PosZ => {
				if (self.z() as usize) < (CHUNK_WIDTH - 1) {
					return Some(LocalBlockPos::new(self.x(), self.y(), self.z() + 1));
				}
			}
			Direction::NegZ => {
				if self.z() > 0 {
					return Some(LocalBlockPos::new(self.x(), self.y(), self.z() - 1));
				}
			}
			_ => {}
		}
		None
	}
	pub fn x(&self) -> u8 {
		(self.xz & 0b11110000) >> 4
	}
	pub fn y(&self) -> u8 {
		self.y
	}
	pub fn z(&self) -> u8 {
		self.xz & 0b1111
	}
}

/// Represents all the blocks in a chunk
///
/// blocks are represented negative to positive,  x, z, y
#[derive(Debug, Clone, Copy)]
pub struct ChunkData {
	pub pos: ChunkPos,
	pub blocks: [BlockData; BLOCKS_PER_CHUNK],
}

/// Represents the difference of a chunk from its generated state
#[derive(Debug, Clone)]
pub struct ChunkDeltaData {
	pub pos: ChunkPos,
	pub blocks: Vec<(LocalBlockPos, BlockData)>,
}
impl ChunkDeltaData {
	pub fn empty(pos: ChunkPos) -> ChunkDeltaData {
		ChunkDeltaData {
			pos,
			blocks: Vec::new(),
		}
	}
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct BlockData {
	/// Block type ID
	pub type_id: BlockTypeID,
}
impl Default for BlockData {
	fn default() -> Self {
		Self {
			type_id: AIR_BLOCK_ID,
		}
	}
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct WorldGenesisData {
	pub seed: u64,
}
