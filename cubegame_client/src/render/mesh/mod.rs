pub mod vert;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages, Device, RenderPass,
};

use vert::Vert;

pub struct Mesh {
	pub n_verts: u32,
	vertex_buffer: wgpu::Buffer,
	pub n_tris: u32,
	index_buffer: wgpu::Buffer,
}
impl Mesh {
	pub fn draw(&self, render_pass: &mut RenderPass) {
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
		render_pass.draw_indexed(0..(self.n_tris * 3), 0, 0..1);
	}

	/// Generates a cube mesh for testing
	pub fn test_cube(device: &Device, x_off: f32) -> Mesh {
		let mut verts: [Vert; 24] = [
			// bottom face
			Vert {
				pos: [-0.5, -0.5, -0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [0.5, -0.5, -0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [0.5, -0.5, 0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [-0.5, -0.5, 0.5],
				tex_coord: [0.0, 0.0],
			},
			// front face
			Vert {
				pos: [-0.5, -0.5, 0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [0.5, -0.5, 0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [0.5, 0.5, 0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [-0.5, 0.5, 0.5],
				tex_coord: [0.0, 0.0],
			},
			// left face
			Vert {
				pos: [-0.5, -0.5, -0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [-0.5, -0.5, 0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [-0.5, 0.5, 0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [-0.5, 0.5, -0.5],
				tex_coord: [0.0, 0.0],
			},
			// back face
			Vert {
				pos: [0.5, -0.5, -0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [-0.5, -0.5, -0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [-0.5, 0.5, -0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [0.5, 0.5, -0.5],
				tex_coord: [0.0, 0.0],
			},
			// right face
			Vert {
				pos: [0.5, -0.5, 0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [0.5, -0.5, -0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [0.5, 0.5, -0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [0.5, 0.5, 0.5],
				tex_coord: [0.0, 0.0],
			},
			// top face
			Vert {
				pos: [-0.5, 0.5, 0.5],
				tex_coord: [0.0, 1.0],
			},
			Vert {
				pos: [0.5, 0.5, 0.5],
				tex_coord: [1.0, 1.0],
			},
			Vert {
				pos: [0.5, 0.5, -0.5],
				tex_coord: [1.0, 0.0],
			},
			Vert {
				pos: [-0.5, 0.5, -0.5],
				tex_coord: [0.0, 0.0],
			},
		];
		for v in verts.iter_mut() {
			v.pos[0] += x_off;
		}
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
		Self {
			n_verts: verts.len() as u32,
			vertex_buffer: device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh vertex buffer (test cube)"),
				contents: bytemuck::cast_slice(&verts),
				usage: BufferUsages::VERTEX,
			}),
			n_tris: indices.len() as u32 / 3,
			index_buffer: device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh index buffer (test cube)"),
				contents: bytemuck::cast_slice(&indices),
				usage: BufferUsages::INDEX,
			}),
		}
	}
}
