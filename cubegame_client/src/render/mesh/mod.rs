pub mod vert;

use crate::render::Renderer;
use cubegame_lib::{BlockTypeID, Direction};
use vert::Vert;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages,
};
use crate::render::texture::atlas::TextureAtlasKey;

pub struct Mesh {
	/// Number of verts
	pub n_verts: u32,
	pub vertex_buffer: wgpu::Buffer,
	/// Number of tris
	pub n_tris: u32,
	pub index_buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}
impl Mesh {
	/// Generates a cube mesh with only certain faces
	pub fn new_block_from_faces(renderer: &Renderer, faces: Direction, x: i32, y: i32, z: i32, block_type_id: BlockTypeID) -> Result<Mesh, ()> {
		// has capacity to fit all faces without reallocating
		let mut n_verts = 0;
		let mut verts: Vec<Vert> = Vec::with_capacity(24);
		let mut n_tris = 0;
		let mut indices: Vec<u32> = Vec::with_capacity(36);

		// Helper function to set up verts and indices for each face
		let mut add_face = |new_verts: &[Vert; 4]| {
			verts.extend_from_slice(new_verts);
			indices.extend_from_slice(&[
				n_verts,
				n_verts + 1,
				n_verts + 2,
				n_verts,
				n_verts + 2,
				n_verts + 3,
			]);
			n_verts += 4;
			n_tris += 2;
		};

		if faces.contains(Direction::PosX) {
			// right face

			add_face(&[
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
			add_face(&[
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
			add_face(&[
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
			add_face(&[
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
			add_face(&[
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
			add_face(&[
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

		let pos = &[x as f32, y as f32, z as f32];
		let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Mesh position buffer"),
			contents: bytemuck::cast_slice(pos),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let rect = match renderer.block_texture_atlas.get_pos_of(TextureAtlasKey::Block(block_type_id)) {
			Some(rect) => rect,
			None => {
				log::error!("Couldn't find position of block texture in texture atlas");
				return Err(());
			},
		};
		let atlas_pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Mesh texture atlas position buffer"),
			contents: bytemuck::cast_slice(rect),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let bind_group = renderer
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &renderer.mesh_bind_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: pos_buffer.as_entire_binding(),
					},
					wgpu::BindGroupEntry {
						binding: 1,
						resource: atlas_pos_buffer.as_entire_binding(),
					},
				],
				label: Some("Mesh local bind group"),
			});

		Ok(Self {
			n_verts,
			vertex_buffer: renderer.device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh vertex buffer"),
				contents: bytemuck::cast_slice(&verts),
				usage: BufferUsages::VERTEX,
			}),
			n_tris,
			index_buffer: renderer.device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh index buffer"),
				contents: bytemuck::cast_slice(&indices),
				usage: BufferUsages::INDEX,
			}),
			bind_group,
		})
	}
}
