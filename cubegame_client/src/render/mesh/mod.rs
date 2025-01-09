pub mod vert;

use crate::render::Renderer;
use cubegame_lib::Direction;
use vert::Vert;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages,
};

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
	pub fn from_faces(renderer: &Renderer, faces: Direction, x: i32, y: i32, z: i32) -> Mesh {
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
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [1.0, 0.0, 1.0], tex_coord: [0.0, 1.0] },
				Vert { pos: [1.0, 0.0, 0.0], tex_coord: [1.0, 1.0] },
				Vert { pos: [1.0, 1.0, 0.0], tex_coord: [1.0, 0.0] },
				Vert { pos: [1.0, 1.0, 1.0], tex_coord: [0.0, 0.0] },
			]);
		}
		if faces.contains(Direction::NegX) {
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [0.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
				Vert { pos: [0.0, 0.0, 1.0], tex_coord: [1.0, 1.0], },
				Vert { pos: [0.0, 1.0, 1.0], tex_coord: [1.0, 0.0], },
				Vert { pos: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
			]);
		}
		if faces.contains(Direction::PosY) {
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [0.0, 1.0, 1.0], tex_coord: [0.0, 1.0], },
				Vert { pos: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0], },
				Vert { pos: [1.0, 1.0, 0.0], tex_coord: [1.0, 0.0], },
				Vert { pos: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
			]);
		}
		if faces.contains(Direction::NegY) {
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [0.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
				Vert { pos: [1.0, 0.0, 0.0], tex_coord: [1.0, 1.0], },
				Vert { pos: [1.0, 0.0, 1.0], tex_coord: [1.0, 0.0], },
				Vert { pos: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0], },
			]);
		}
		if faces.contains(Direction::PosZ) {
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0], },
				Vert { pos: [1.0, 0.0, 1.0], tex_coord: [1.0, 1.0], },
				Vert { pos: [1.0, 1.0, 1.0], tex_coord: [1.0, 0.0], },
				Vert { pos: [0.0, 1.0, 1.0], tex_coord: [0.0, 0.0], },
			]);
		}
		if faces.contains(Direction::NegZ) {
			#[rustfmt::skip]
			add_face(&[
				Vert { pos: [1.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
				Vert { pos: [0.0, 0.0, 0.0], tex_coord: [1.0, 1.0], },
				Vert { pos: [0.0, 1.0, 0.0], tex_coord: [1.0, 0.0], },
				Vert { pos: [1.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
			]);
		}

		let pos = &[x as f32, y as f32, z as f32];
		let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Mesh position buffer"),
			contents: bytemuck::cast_slice(pos),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let bind_group = renderer
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &renderer.mesh_bind_group_layout,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: pos_buffer.as_entire_binding(),
				}],
				label: Some("Mesh local bind group"),
			});

		Self {
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
		}
	}

	/// Generates a new cube mesh at a given position
	pub fn new_block(renderer: &Renderer, x: i32, y: i32, z: i32) -> Mesh {
		#[rustfmt::skip]
		let verts: [Vert; 24] = [
			// bottom face
			Vert { pos: [0.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [1.0, 0.0, 0.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [1.0, 0.0, 1.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [0.0, 0.0, 1.0], tex_coord: [0.0, 0.0], },
			// front face
			Vert { pos: [0.0, 0.0, 1.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [1.0, 0.0, 1.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [1.0, 1.0, 1.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [0.0, 1.0, 1.0], tex_coord: [0.0, 0.0], },
			// left face
			Vert { pos: [0.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [0.0, 0.0, 1.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [0.0, 1.0, 1.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
			// back face
			Vert { pos: [1.0, 0.0, 0.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [0.0, 0.0, 0.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [0.0, 1.0, 0.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [1.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
			// right face
			Vert { pos: [1.0, 0.0, 1.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [1.0, 0.0, 0.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [1.0, 1.0, 0.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [1.0, 1.0, 1.0], tex_coord: [0.0, 0.0], },
			// top face
			Vert { pos: [0.0, 1.0, 1.0], tex_coord: [0.0, 1.0], },
			Vert { pos: [1.0, 1.0, 1.0], tex_coord: [1.0, 1.0], },
			Vert { pos: [1.0, 1.0, 0.0], tex_coord: [1.0, 0.0], },
			Vert { pos: [0.0, 1.0, 0.0], tex_coord: [0.0, 0.0], },
		];
		#[rustfmt::skip]
        let indices: [u32; 36] = [
            0, 1, 2,
			0, 2, 3,
			4, 5, 6,
			4, 6, 7,
			8, 9, 10,
			8, 10, 11,
			12, 13, 14,
			12, 14, 15,
			16, 17, 18,
			16, 18, 19,
			20, 21, 22,
			20, 22, 23,
        ];

		let pos = &[x as f32, y as f32, z as f32];
		let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Mesh position buffer"),
			contents: bytemuck::cast_slice(pos),
			usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
		});
		let bind_group = renderer
			.device
			.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &renderer.mesh_bind_group_layout,
				entries: &[wgpu::BindGroupEntry {
					binding: 0,
					resource: pos_buffer.as_entire_binding(),
				}],
				label: Some("Mesh local bind group"),
			});

		Self {
			n_verts: verts.len() as u32,
			vertex_buffer: renderer.device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh vertex buffer (test cube)"),
				contents: bytemuck::cast_slice(&verts),
				usage: BufferUsages::VERTEX,
			}),
			n_tris: indices.len() as u32 / 3,
			index_buffer: renderer.device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh index buffer (test cube)"),
				contents: bytemuck::cast_slice(&indices),
				usage: BufferUsages::INDEX,
			}),
			bind_group,
		}
	}
}
