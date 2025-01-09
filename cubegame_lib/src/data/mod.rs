/// Represents a chunk
///
/// blocks are represented negative to positive x, z, y
#[derive(Debug, Clone)]
pub struct ChunkData {
	pub blocks: Vec<BlockData>,
}

#[derive(Debug, Copy, Clone)]
pub struct BlockData {
	/// Block type
	pub ty: u8,
}