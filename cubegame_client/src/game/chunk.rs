use cubegame_lib::{worldgen, ChunkData, ChunkDeltaData, LocalBlockPos, CHUNK_WIDTH};

use crate::render::mesh::Mesh;

pub struct LoadedChunk {
	/// Chunk data: blocks
	pub data: Box<ChunkData>,
	/// This chunks meshes (one for each texture)
	pub meshes: Vec<Mesh>,
	pub needs_remesh: bool,
}
impl LoadedChunk {
	/// Loads new chunk from chunk data
	///
	/// (Does not generate meshes)
	pub fn load_from_delta(delta: ChunkDeltaData) -> LoadedChunk {
		let chunk_pos = delta.pos;

		// data from the world generator
		let mut chunk = worldgen::generate_chunk(chunk_pos);

		// overwriting block data with blocks from chunk delta
		for (pos, data) in delta.blocks {
			chunk.blocks[pos.to_index()] = data;
		}

		LoadedChunk {
			data: chunk,
			meshes: Vec::new(),
			needs_remesh: true,
		}
	}

	/// Converts local block coordinates in this chunk to world coordinates
	fn to_world_coords(&self, block_pos: LocalBlockPos) -> (i32, i32, i32) {
		let chunk_w = CHUNK_WIDTH as i32;
		(
			(block_pos.x() as i32) + self.data.pos.x * chunk_w,
			block_pos.y() as i32,
			(block_pos.z() as i32) + self.data.pos.z * chunk_w,
		)
	}
}
