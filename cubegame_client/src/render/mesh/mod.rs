pub mod vert;

use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages, Device, RenderPass,
};

use vert::Vert;

pub struct Mesh {
	/*
	pub verts: Box<[Vert]>,
	pub tris: Box<[u32]>*/
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
	pub fn test_cube(device: &Device) -> Mesh {
		#[rustfmt::skip]
        let verts = &[
            Vert { pos: [-0.5, -0.5, -0.5], },
            Vert { pos: [0.5, -0.5, -0.5], },
            Vert { pos: [0.5, -0.5, 0.5], },
            Vert { pos: [-0.5, -0.5, 0.5], },
            Vert { pos: [-0.5, 0.5, -0.5], },
            Vert { pos: [0.5, 0.5, -0.5], },
            Vert { pos: [0.5, 0.5, 0.5], },
            Vert { pos: [-0.5, 0.5, 0.5], },
        ];
		#[rustfmt::skip]
        let indices = &[
            0, 1, 2,
			0, 2, 3,
			0, 4, 5,
			0, 5, 1,
			1, 5, 6,
			1, 6, 2,
			2, 6, 7,
			2, 7, 3,
			3, 7, 4,
			3, 4, 0,
			4, 6, 5,
			4, 7, 6,
        ];
		Self {
			n_verts: 8,
			vertex_buffer: device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh vertex buffer (test cube)"),
				contents: bytemuck::cast_slice(verts),
				usage: BufferUsages::VERTEX,
			}),
			n_tris: 12,
			index_buffer: device.create_buffer_init(&BufferInitDescriptor {
				label: Some("Mesh index buffer (test cube)"),
				contents: bytemuck::cast_slice(indices),
				usage: BufferUsages::INDEX,
			}),
		}
	}
}
