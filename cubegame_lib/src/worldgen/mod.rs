use crate::{BlockData, ChunkData, ChunkPos, LocalBlockPos, BLOCKS_PER_CHUNK};
// TODO implement real world generation

/// Generates simple chunk for testing
pub fn generate_chunk(pos: ChunkPos) -> Box<ChunkData> {
	let mut chunk = Box::new(ChunkData {
		pos,
		blocks: [BlockData::default(); BLOCKS_PER_CHUNK],
	});

	for i in 0..BLOCKS_PER_CHUNK {
		// generating 3 high stone floor
		let pos = LocalBlockPos::from_index(i);
		if pos.y() < 3 {
			chunk.blocks[i] = BlockData { type_id: 2 };
		} else if pos.y() < 6 {
			chunk.blocks[i] = BlockData { type_id: 3 };
		} else if pos.y() == 6 {
			chunk.blocks[i] = BlockData { type_id: 4 };
		}
	}

	return chunk;
}
