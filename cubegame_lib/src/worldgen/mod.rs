use crate::{
	BlockData, ChunkData, ChunkPos, LocalBlockPos, WorldGenesisData, BLOCKS_PER_CHUNK, CHUNK_WIDTH,
};
use noise::NoiseFn;

/// Generates simple chunk for testing
pub fn generate_chunk(gen: &WorldGenesisData, pos: ChunkPos) -> Box<ChunkData> {
	// create empty chunk
	let mut chunk = Box::new(ChunkData {
		pos,
		blocks: [BlockData::default(); BLOCKS_PER_CHUNK],
	});

	let rng = noise::Simplex::new(gen.seed);

	let offset_x = CHUNK_WIDTH as f64 * pos.x as f64;
	let offset_z = CHUNK_WIDTH as f64 * pos.z as f64;
	for x in 0..CHUNK_WIDTH {
		for z in 0..CHUNK_WIDTH {
			let sample_x = x as f64 + offset_x;
			let sample_z = z as f64 + offset_z;
			let mut floor_height = rng.get([sample_x / 60.0, sample_z / 60.0]) + 1.0;
			floor_height *= 10.0;
			floor_height += 20.0;
			if floor_height.is_nan() {
				floor_height = 100.0;
			}

			let floor_y = floor_height as u8;
			let pos = LocalBlockPos::new(x as u8, floor_y, z as u8);
			// grass on top
			chunk.blocks[pos.to_index()] = BlockData { type_id: 4 };
			// fill with stone
			for y in 0..floor_y {
				let pos = LocalBlockPos::new(x as u8, y, z as u8);
				chunk.blocks[pos.to_index()] = BlockData { type_id: 2 };
			}
			// couple layers of dirt underneath grass
			for y in (floor_y - 3)..floor_y {
				let pos = LocalBlockPos::new(x as u8, y, z as u8);
				chunk.blocks[pos.to_index()] = BlockData { type_id: 3 };
			}
		}
	}

	return chunk;
}
