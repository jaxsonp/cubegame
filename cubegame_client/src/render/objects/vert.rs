use wgpu::{BufferAddress, VertexBufferLayout};

/// Stores information about each vertex in a mesh
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MeshVert {
	pub pos: [f32; 3],
	pub tex_coord: [f32; 2],
}
impl MeshVert {
	pub fn buffer_layout() -> VertexBufferLayout<'static> {
		VertexBufferLayout {
			array_stride: size_of::<Self>() as BufferAddress,
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineVert {
	pub pos: [f32; 3],
}
impl LineVert {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		LineVert { pos: [x, y, z] }
	}
	pub fn buffer_layout() -> VertexBufferLayout<'static> {
		VertexBufferLayout {
			array_stride: size_of::<Self>() as BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			// buffers passed to shader:
			attributes: &[wgpu::VertexAttribute {
				// pos
				offset: 0,
				shader_location: 0,
				format: wgpu::VertexFormat::Float32x3,
			}],
		}
	}
}
