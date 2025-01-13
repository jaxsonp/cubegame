use std::collections::HashMap;

use super::{Mesh, Vert};
use crate::render::texture::atlas::TextureAtlasKey;
use cubegame_lib::{blocks::AIR_BLOCK_ID, ChunkData, Direction, LocalBlockPos, CHUNK_WIDTH};

/// Turns a chunk into meshes
///
/// Current implementation creates one conjoined mesh per texture
pub fn mesh_chunk(data: &ChunkData) -> Vec<Mesh> {
	let chunk_pos = data.pos;
	let mut total_verts = 0;
	let mut total_tris = 0;

	// a list of faces at each block pos for each texture
	let mut meshes: HashMap<TextureAtlasKey, Vec<(LocalBlockPos, Direction)>> = HashMap::new();

	for (i, block) in data.blocks.iter().enumerate() {
		if block.type_id == AIR_BLOCK_ID {
			continue;
		}
		let local_pos = LocalBlockPos::from_index(i);

		// optimization: choosing which faces to render
		let mut faces: Direction = Direction::all_flags();
		for (_, direction) in Direction::flags() {
			// for each direction, check if there is a neighbor in this chunk, and check if that neighbor is air
			if let Some(neighbor) = local_pos.get_neighbor(*direction) {
				if data.blocks[neighbor.to_index()].type_id != AIR_BLOCK_ID {
					faces &= direction.not();
				}
			}
		}

		let tex_key = TextureAtlasKey::Block(block.type_id);
		if meshes.contains_key(&tex_key) {
			meshes.get_mut(&tex_key).unwrap().push((local_pos, faces));
		} else {
			meshes.insert(tex_key, vec![(local_pos, faces)]);
		}
	}

	let pos_offset = &[
		chunk_pos.x as f32 * CHUNK_WIDTH as f32,
		0.0,
		chunk_pos.z as f32 * CHUNK_WIDTH as f32,
	];
	// now we gotta turn each list of faces into a mesh
	let meshes: Vec<Mesh> = meshes
		.into_iter()
		.map(|(tex_key, faces_list)| {
			let mut verts: Vec<Vert> = Vec::with_capacity(24);
			let mut indices: Vec<u32> = Vec::with_capacity(36);

			for (pos, faces) in faces_list.into_iter() {
				// Helper function to set up verts and indices for each face
				let mut add_face = |mut new_verts: [Vert; 4]| {
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

				if faces.contains(Direction::PosX) {
					// right face

					add_face([
						Vert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [1.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Direction::NegX) {
					add_face([
						Vert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Direction::PosY) {
					add_face([
						Vert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [1.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Direction::NegY) {
					add_face([
						Vert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Direction::PosZ) {
					add_face([
						Vert {
							pos: [0.0, 0.0, 1.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [1.0, 0.0, 1.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [1.0, 1.0, 1.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
							pos: [0.0, 1.0, 1.0],
							tex_coord: [0.0, 0.0],
						},
					]);
				}
				if faces.contains(Direction::NegZ) {
					add_face([
						Vert {
							pos: [1.0, 0.0, 0.0],
							tex_coord: [0.0, 1.0],
						},
						Vert {
							pos: [0.0, 0.0, 0.0],
							tex_coord: [1.0, 1.0],
						},
						Vert {
							pos: [0.0, 1.0, 0.0],
							tex_coord: [1.0, 0.0],
						},
						Vert {
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
