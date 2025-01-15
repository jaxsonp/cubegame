use cubegame_lib::{worldgen, ChunkData, ChunkDeltaData, WorldGenesisData};

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
		let mut chunk = worldgen::generate_chunk(&WorldGenesisData::default(), chunk_pos);

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
}
