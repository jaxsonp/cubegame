use crate::render::objects::lines::LineVert;
use crate::render::objects::mesh::vert::MeshVert;
use crate::render::objects::{Lines, Mesh};
use crate::render::texture::atlas::TextureAtlasKey;
use crate::CHUNK_BORDER_COLOR;
use cubegame_lib::blocks::{BlockTextureLayout, BlockType};
use cubegame_lib::{
	blocks::AIR_BLOCK_ID, ChunkData, Direction, Directions, LocalBlockPos, CHUNK_WIDTH,
	WORLD_HEIGHT,
};
use std::collections::HashMap;

/// Turns a chunk into meshes
///
/// Current implementation creates one conjoined objects per texture
/// TODO remove redundant rendering on the sides of the chunk
/// TODO randomize texture orientation
pub fn generate_chunk_meshes(data: &ChunkData) -> Vec<Mesh> {
	let chunk_pos = data.pos;
	let mut total_verts = 0;
	let mut total_tris = 0;

	// a list of faces at each block pos for each texture
	let mut meshes: HashMap<TextureAtlasKey, Vec<(LocalBlockPos, Directions)>> = HashMap::new();

	for (i, block) in data.blocks.iter().enumerate() {
		if block.type_id == AIR_BLOCK_ID {
			continue;
		}
		let local_pos = LocalBlockPos::from_index(i);

		// optimization: choosing which faces to render
		let mut faces = Directions::all_flags();
		// for each direction, check if there is a neighbor in this chunk, and check if that neighbor is air
		if let Some(neighbor) = local_pos.get_neighbor(Direction::PosX) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::PosX;
			}
		}
		if let Some(neighbor) = local_pos.get_neighbor(Direction::NegX) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::NegX;
			}
		}
		if let Some(neighbor) = local_pos.get_neighbor(Direction::PosY) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::PosY;
			}
		}
		if let Some(neighbor) = local_pos.get_neighbor(Direction::NegY) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::NegY;
			}
		}
		if let Some(neighbor) = local_pos.get_neighbor(Direction::PosZ) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::PosZ;
			}
		}
		if let Some(neighbor) = local_pos.get_neighbor(Direction::NegZ) {
			if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
				faces ^= Directions::NegZ;
			}
		}

		let block_type = BlockType::from_id(block.type_id);
		let mut insert_faces = |tex_key, faces| {
			if meshes.contains_key(&tex_key) {
				meshes.get_mut(&tex_key).unwrap().push((local_pos, faces));
			} else {
				meshes.insert(tex_key, vec![(local_pos, faces)]);
			}
		};
		match block_type.texture_layout {
			// dont care about orientation when its a uniform block
			BlockTextureLayout::Uniform(_) => {
				insert_faces(TextureAtlasKey::Block(block.type_id), faces);
			}
			// if its not uniform, face matters
			_ => {
				for (_, face) in Directions::flags() {
					if faces.contains(*face) {
						insert_faces(
							TextureAtlasKey::BlockFace(block.type_id, (*face).into()),
							*face,
						);
					}
				}
			}
		}
	}

	let pos_offset = [
		chunk_pos.x as f32 * CHUNK_WIDTH as f32,
		0.0,
		chunk_pos.z as f32 * CHUNK_WIDTH as f32,
	];
	// now we gotta turn each list of faces into a mesh
	let meshes: Vec<Mesh> = meshes
		.into_iter()
		.map(|(tex_key, faces_list)| {
			let mut verts: Vec<MeshVert> = Vec::with_capacity(24);
			let mut indices: Vec<u32> = Vec::with_capacity(36);

			for (pos, faces) in faces_list.into_iter() {
				// Helper function to set up verts and indices for each face
				let mut add_face = |mut new_verts: [MeshVert; 4]| {
					let n_verts = verts.len() as u32;
					indices.extend_from_slice(&[
						n_verts,
						n_verts + 1,
						n_verts + 2,
						n_verts,
						n_verts + 2,
						n_verts + 3,
					]);
					for v in new_verts.iter_mut() {
						v.pos[0] += pos.x() as f32;
						v.pos[1] += pos.y() as f32;
						v.pos[2] += pos.z() as f32;
					}
					verts.extend_from_slice(&new_verts);
				};

				if faces.contains(Directions::PosX) {
					// right face

					add_face([
						MeshVert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Directions::NegX) {
					add_face([
						MeshVert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Directions::PosY) {
					add_face([
						MeshVert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Directions::NegY) {
					add_face([
						MeshVert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Directions::PosZ) {
					add_face([
						MeshVert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Directions::NegZ) {
					add_face([
						MeshVert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						MeshVert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						MeshVert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						MeshVert {
							pos: [1.0, 1.0, 0.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
			}
			total_verts += verts.len();
			total_tris += verts.len();
			Mesh::new(verts, indices, pos_offset, tex_key)
		})
		.collect();

	/*log::debug!(
		"Remeshed chunk at {} - {} meshes, {} verts, {} tris",
		chunk_pos,
		meshes.len(),
		total_verts,
		total_tris
	);*/
	meshes
}

pub fn generate_chunk_border_lines(data: &ChunkData) -> Lines {
	let width = CHUNK_WIDTH as f32;
	let height = WORLD_HEIGHT as f32;
	let verts: Vec<LineVert> = vec![
		LineVert::new(0.0, 0.0, 0.0),
		LineVert::new(0.0, height, 0.0),
		LineVert::new(width, 0.0, 0.0),
		LineVert::new(width, height, 0.0),
		LineVert::new(0.0, 0.0, width),
		LineVert::new(0.0, height, width),
		LineVert::new(width, 0.0, width),
		LineVert::new(width, height, width),
	];
	let x = data.pos.x as f32 * width;
	let z = data.pos.z as f32 * width;
	Lines::new(verts, [x, 0.0, z], CHUNK_BORDER_COLOR)
}
