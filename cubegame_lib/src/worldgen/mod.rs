use crate::{BlockData, ChunkData, ChunkPos, BLOCKS_PER_CHUNK, CHUNK_WIDTH};
// TODO implement real world generation

/// Generates simple chunk for testing
pub fn generate_chunk(pos: ChunkPos) -> Box<ChunkData> {
	let mut chunk = Box::new(ChunkData {
		pos,
		blocks: [BlockData { type_id: 0 }; BLOCKS_PER_CHUNK],
	});

	for i in 0..BLOCKS_PER_CHUNK {
		// generating 3 high stone floor
		if i < (3 * CHUNK_WIDTH * CHUNK_WIDTH) {
			chunk.blocks[i] = BlockData { type_id: 2 };
		}
	}

	return chunk;
}
