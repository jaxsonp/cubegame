use wgpu::{BufferAddress, VertexBufferLayout};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vert {
	pub pos: [f32; 3],
	pub tex_coord: [f32; 2],
}
impl Vert {
	/// Buffer descriptor for vertices
	pub fn buffer_layout() -> VertexBufferLayout<'static> {
		VertexBufferLayout {
			array_stride: size_of::<Vert>() as BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			// buffers passed to shader:
			attributes: &[
				wgpu::VertexAttribute {
					// pos
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					// tex_coord
					offset: size_of::<[f32; 3]>() as BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x2,
				},
			],
		}
	}
}
