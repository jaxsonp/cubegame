mod vert;

use crate::render::Renderer;
pub use vert::LineVert;
use wgpu::util::BufferInitDescriptor;
use wgpu::util::DeviceExt;
use wgpu::BufferUsages;

/// Represents whether or not these lines are loaded into buffers, and if not contains the data to do so
enum LinesRenderState {
	Loaded(LinesRenderObjects),
	Unloaded {
		verts: Vec<LineVert>,
		pos: [f32; 3],
		color: [f32; 3],
	},
}

pub struct LinesRenderObjects {
	pub vertex_buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

/// Represents a collection of debug lines
pub struct Lines {
	pub n_lines: u32,
	render_state: LinesRenderState,
}
impl Lines {
	pub fn new(verts: Vec<LineVert>, pos: [f32; 3], color: [f32; 3]) -> Self {
		Lines {
			n_lines: verts.len() as u32 / 2,
			render_state: LinesRenderState::Unloaded { verts, pos, color },
		}
	}

	/// Gets this meshes render objects, eg its buffers and bind group
	pub fn get_render_objs(&self) -> Option<&LinesRenderObjects> {
		if let LinesRenderState::Loaded(objs) = &self.render_state {
			Some(objs)
		} else {
			None
		}
	}

	/// Creates buffers and bind group if it hasn't been loaded already
	pub fn load_buffers(&mut self, renderer: &Renderer) {
		match &self.render_state {
			LinesRenderState::Unloaded { verts, pos, color } => {
				// position offset of these lines
				let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Lines position buffer"),
					contents: bytemuck::cast_slice(pos),
					usage: BufferUsages::UNIFORM,
				});
				// position of texture in the texture atlas
				let color_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Lines color buffer"),
					contents: bytemuck::cast_slice(color),
					usage: BufferUsages::UNIFORM,
				});
				let bind_group = renderer
					.device
					.create_bind_group(&wgpu::BindGroupDescriptor {
						layout: &renderer.line_rendering_pipeline.local_bind_group_layout,
						entries: &[
							wgpu::BindGroupEntry {
								binding: 0,
								resource: pos_buffer.as_entire_binding(),
							},
							wgpu::BindGroupEntry {
								binding: 1,
								resource: color_buffer.as_entire_binding(),
							},
						],
						label: Some("Lines local bind group"),
					});
				let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Lines vertex buffer"),
					contents: bytemuck::cast_slice(&verts),
					usage: BufferUsages::VERTEX,
				});

				let render_objs = LinesRenderObjects {
					vertex_buffer,
					bind_group,
				};
				self.render_state = LinesRenderState::Loaded(render_objs);
			}
			LinesRenderState::Loaded(_) => {}
		}
	}
}
