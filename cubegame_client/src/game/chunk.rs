use crate::render::mesh::Mesh;
use crate::render::Renderer;
use cubegame_lib::{blocks::BlockType, data::ChunkData, BlockPos, BlockTypeID, CHUNK_WIDTH};

pub struct LoadedChunk {
	/// X positional index of this chunk
	x_pos: i32,
	/// Z positional index of this chunk
	z_pos: i32,
	/// Marks whether or not blocks in this chunk have been updated (needs to remesh)
	pub needs_remesh: bool,
	blocks: Vec<LoadedBlock>,
	pub meshes: Vec<Mesh>,
}
impl LoadedChunk {
	/// Loads new chunk from ChunkData with an x and z position
	///
	/// (Does not generate meshes)
	pub fn new(chunk_data: ChunkData, x_pos: i32, z_pos: i32) -> LoadedChunk {
		let mut blocks = Vec::with_capacity(chunk_data.blocks.len());

		// reading blocks from chunk data
		for (i, block_data) in chunk_data.blocks.iter().enumerate() {
			let x = i % CHUNK_WIDTH;
			let z = (i / CHUNK_WIDTH) % CHUNK_WIDTH;
			let y = i / (CHUNK_WIDTH * CHUNK_WIDTH);
			let block = LoadedBlock::new(x as u8, y as u8, z as u8, block_data.ty);
			blocks.push(block);
		}

		LoadedChunk {
			x_pos,
			z_pos,
			needs_remesh: true,
			blocks,
			meshes: Vec::new(),
		}
	}

	/// Turns this chunk's loaded blocks into meshes
	pub fn regenerate_meshes(&mut self, renderer: &Renderer) {
		// TODO optimize
		self.meshes.clear();
		for block in self.blocks.iter() {
			if block.ty.is_air() {
				continue;
			}
			let (x, y, z) =
				self.to_world_coords(block.pos);
			self.meshes.push(Mesh::new_block(renderer, x, y, z));
		}
		self.needs_remesh = false;
	}

	/// Converts local block coordinates to world coordinates
	fn to_world_coords(&self, pos: BlockPos) -> (i32, i32, i32) {
		let chunk_w = CHUNK_WIDTH as i32;
		((pos.x() as i32) + self.x_pos * chunk_w, pos.y() as i32, (pos.z() as i32) + self.z_pos * chunk_w)
	}
}

/// Represents a block within a loaded chunk
pub struct LoadedBlock {
	pub pos: BlockPos,
	pub ty: BlockType,
}
impl LoadedBlock {
	pub fn new(x: u8, y: u8, z: u8, type_id: BlockTypeID) -> LoadedBlock {
		LoadedBlock {
			pos: BlockPos::new(x, y, z),
			ty: BlockType::from_id(type_id),
		}
	}
}
