use crate::data::BlockData;
use crate::{data::ChunkData, BLOCKS_PER_CHUNK, CHUNK_WIDTH};
// TODO implement world generation

/// Generates simple chunk for testing
pub fn generate_chunk() -> ChunkData {
	let mut blocks: Vec<BlockData> = Vec::with_capacity(BLOCKS_PER_CHUNK);
	for i in 0..BLOCKS_PER_CHUNK {
		blocks.push(if i < (3 * CHUNK_WIDTH * CHUNK_WIDTH) {
			BlockData {
				ty: 2
			}
		} else {
			BlockData {
				ty: 0
			}
		});
	}

	ChunkData { blocks }
}
