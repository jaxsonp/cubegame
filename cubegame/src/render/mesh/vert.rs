


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vert {
	pub pos: [f32; 3],
}
impl Vert {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { pos: [x, y, z] }
	}
	
	/// Buffer descriptor for vertices
	pub fn desc() -> wgpu::VertexBufferLayout<'static> {
		wgpu::VertexBufferLayout {
			array_stride: size_of::<Vert>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[ // buffers passed to shader:
				wgpu::VertexAttribute { // pos
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3,
				},
			]
		}

	}
}
