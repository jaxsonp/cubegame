use crate::render::{mesh::Mesh, Renderer};
use cubegame_lib::blocks::AIR_BLOCK_ID;
use cubegame_lib::{
	worldgen, BlockData, ChunkData, ChunkDeltaData, LocalBlockPos, BLOCKS_PER_CHUNK, CHUNK_WIDTH,
};

pub struct LoadedChunk {
	/// Chunk data: blocks
	pub data: ChunkData,
	pub meshes: Vec<Mesh>,
	/// Marks whether or not blocks in this chunk have been updated (needs to remesh)
	pub needs_remesh: bool,
}
impl LoadedChunk {
	/// Loads new chunk from chunk data
	///
	/// (Does not generate meshes)
	pub fn load_from_delta(delta: ChunkDeltaData) -> LoadedChunk {
		let chunk_pos = delta.pos;

		// data from the world generator
		let worldgen_data = worldgen::generate_chunk(chunk_pos);

		let mut block_data: [BlockData; BLOCKS_PER_CHUNK] = worldgen_data.blocks;

		// overwriting block data with blocks from chunk delta
		for (pos, data) in delta.blocks {
			block_data[pos.to_index()] = data;
		}

		LoadedChunk {
			data: ChunkData {
				pos: chunk_pos,
				blocks: block_data,
			},
			meshes: Vec::new(),
			needs_remesh: true,
		}
	}

	/// Turns this chunk's loaded blocks into meshes
	pub fn regenerate_meshes(&mut self, renderer: &Renderer) {
		// TODO optimize
		self.meshes.clear();
		let mut total_verts = 0;
		let mut total_tris = 0;
		for (i, block) in self.data.blocks.iter().enumerate() {
			if block.type_id == AIR_BLOCK_ID {
				continue;
			}
			let (x, y, z) = self.to_world_coords(LocalBlockPos::from_index(i));

			let new_mesh = Mesh::new_block(renderer, x, y, z);
			total_verts += new_mesh.n_verts;
			total_tris += new_mesh.n_tris;
			self.meshes.push(new_mesh);
		}
		self.needs_remesh = false;
		log::debug!(
			"Remeshed chunk at ({}, {}) - {} verts, {} tris",
			self.data.pos.x(),
			self.data.pos.z(),
			total_verts,
			total_tris
		);
	}

	/// Converts local block coordinates to world coordinates
	fn to_world_coords(&self, block_pos: LocalBlockPos) -> (i32, i32, i32) {
		let chunk_w = CHUNK_WIDTH as i32;
		(
			(block_pos.x() as i32) + self.data.pos.x() * chunk_w,
			block_pos.y() as i32,
			(block_pos.z() as i32) + self.data.pos.z() * chunk_w,
		)
	}
}

/*/// Represents a block within a loaded chunk
pub struct LoadedBlock {
	pub pos: LocalBlockPos,
	pub type_id: BlockTypeID,
}
impl LoadedBlock {
	pub fn new(pos: LocalBlockPos, type_id: BlockTypeID) -> LoadedBlock {
		LoadedBlock {
			pos,
			type_id,
		}
	}
}*/
