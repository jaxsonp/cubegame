pub mod blocks;
/// Module for shared data type specifications
pub mod data;
pub mod worldgen;

// constants
pub const WORLD_HEIGHT: usize = 256;
pub const CHUNK_WIDTH: usize = 16;
pub const BLOCKS_PER_CHUNK: usize = CHUNK_WIDTH * CHUNK_WIDTH * WORLD_HEIGHT;

// types
pub type BlockTypeID = u8;
/// Chunk indexing position
pub type ChunkPos = (i32, i32);
#[derive(PartialEq, Copy, Clone, Debug, Eq)]
/// Local block position within a chunk
pub struct BlockPos {
	/// Represents both the x and z pos
	/// Most significant 4 bits are z, least significant 4 bits are x
	xz: u8,
	y: u8,
}
impl BlockPos {
	pub fn new(x: u8, y: u8, z: u8) -> BlockPos {
		BlockPos {
			xz: ((z & 0b1111) << 4) + (x & 0b1111),
			y,
		}
	}
	pub fn x(&self) -> u8 {
		self.xz & 0b1111
	}
	pub fn y(&self) -> u8 {
		self.y
	}
	pub fn z(&self) -> u8 {
		(self.xz & 0b11110000) >> 4
	}
}
