use crate::render::{mesh::Mesh, Renderer};
use cubegame_lib::{
	blocks::AIR_BLOCK_ID, worldgen, BlockData, ChunkData, ChunkDeltaData, Direction, LocalBlockPos,
	BLOCKS_PER_CHUNK, CHUNK_WIDTH,
};

pub struct LoadedChunk {
	/// Chunk data: blocks
	pub data: Box<ChunkData>,
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

	/// Turns this chunk's loaded blocks into meshes (aka remesh)
	pub fn regenerate_meshes(&mut self, renderer: &Renderer) -> Result<(), ()> {
		// TODO remesh asynchronously
		self.meshes.clear();
		let mut total_verts = 0;
		let mut total_tris = 0;
		for (i, block) in self.data.blocks.iter().enumerate() {
			if block.type_id == AIR_BLOCK_ID {
				continue;
			}
			let local_pos = LocalBlockPos::from_index(i);
			let (x, y, z) = self.to_world_coords(local_pos);

			// optimization: choosing which faces to render
			let mut faces: Direction = Direction::all_flags();
			for (_, direction) in Direction::flags() {
				// for each direction, check if there is a neighbor in this chunk, and check if that neighbor is air
				if let Some(neighbor) = local_pos.get_neighbor(*direction) {
					if self.data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
						faces &= direction.not();
					}
				}
			}

			let new_mesh = Mesh::new_block_from_faces(renderer, faces, x, y, z, block.type_id)?;
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
		return Ok(());
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
