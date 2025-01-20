use crate::render::mesher;
use crate::render::objects::{Lines, Mesh};
use cubegame_lib::{worldgen, ChunkData, ChunkDeltaData, WorldGenesisData};

pub struct LoadedChunk {
	/// Chunk data: blocks
	pub data: Box<ChunkData>,
	/// This chunks meshes (one for each texture)
	pub meshes: Vec<Mesh>,
	/// This chunks debug lines
	pub border_lines: Lines,
	pub needs_remesh: bool,
}
impl LoadedChunk {
	/// Loads new chunk from chunk data
	///
	/// (Does not generate meshes) (but does generate chunk borders cus those never change)
	pub fn load_from_delta(delta: ChunkDeltaData) -> LoadedChunk {
		let chunk_pos = delta.pos;

		// data from the world generator
		let mut chunk = worldgen::generate_chunk(&WorldGenesisData::default(), chunk_pos);
		let border_lines = mesher::generate_chunk_border_lines(chunk.as_ref());

		// overwriting block data with blocks from chunk delta
		for (pos, data) in delta.blocks {
			chunk.blocks[pos.to_index()] = data;
		}

		LoadedChunk {
			data: chunk,
			meshes: Vec::new(),
			border_lines,
			needs_remesh: true,
		}
	}
}
