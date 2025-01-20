use crate::render::texture::atlas::TextureAtlasKey;
use crate::render::Renderer;
pub use vert::MeshVert;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages,
};

pub mod vert;

/// Represents whether the objects has been loaded into buffers and such, and if not, contains the data to load it
enum MeshRenderState {
	Loaded(MeshRenderObjects),
	Unloaded {
		verts: Vec<MeshVert>,
		indices: Vec<u32>,
		pos: [f32; 3],
		texture: TextureAtlasKey,
	},
}

/// Everything required to render a objects, used by the world rendering pipeline
pub struct MeshRenderObjects {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
	/// Number of verts
	pub n_verts: u32,
	/// Number of tris
	pub n_tris: u32,
	/// Whether this objects has been loaded
	render_state: MeshRenderState,
}
impl Mesh {
	pub fn new(
		verts: Vec<MeshVert>,
		indices: Vec<u32>,
		pos: [f32; 3],
		texture: TextureAtlasKey,
	) -> Mesh {
		Mesh {
			n_verts: verts.len() as u32,
			n_tris: indices.len() as u32 / 3,
			render_state: MeshRenderState::Unloaded {
				verts,
				indices,
				pos,
				texture,
			},
		}
	}

	/// Gets this meshes render objects, eg its buffers and bind group
	pub fn get_render_objs(&self) -> Option<&MeshRenderObjects> {
		if let MeshRenderState::Loaded(objs) = &self.render_state {
			Some(objs)
		} else {
			None
		}
	}

	/// Creates buffers and bind group for this mesh if it hasn't been loaded already
	pub fn load_buffers(&mut self, renderer: &Renderer) {
		match &self.render_state {
			MeshRenderState::Unloaded {
				verts,
				indices,
				pos,
				texture,
			} => {
				// position offset of this whole mesh
				let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh position buffer"),
					contents: bytemuck::cast_slice(pos),
					usage: BufferUsages::UNIFORM,
				});
				let atlas_pos_rect = renderer
					.world_rendering_pipeline
					.block_texture_atlas
					.get_pos_of(*texture)
					.unwrap_or_else(|| {
						renderer
							.world_rendering_pipeline
							.block_texture_atlas
							.get_pos_of(TextureAtlasKey::Null)
							.unwrap()
					});
				// position of texture in the texture atlas
				let atlas_pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh texture atlas position buffer"),
					contents: bytemuck::cast_slice(atlas_pos_rect),
					usage: BufferUsages::UNIFORM,
				});
				let bind_group = renderer
					.device
					.create_bind_group(&wgpu::BindGroupDescriptor {
						layout: &renderer.world_rendering_pipeline.local_bind_group_layout,
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
				let vertex_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh vertex buffer"),
					contents: bytemuck::cast_slice(&verts),
					usage: BufferUsages::VERTEX,
				});
				let index_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh index buffer"),
					contents: bytemuck::cast_slice(&indices),
					usage: BufferUsages::INDEX,
				});

				let render_objs = MeshRenderObjects {
					vertex_buffer,
					index_buffer,
					bind_group,
				};
				self.render_state = MeshRenderState::Loaded(render_objs);
			}
			MeshRenderState::Loaded(_) => {}
		}
	}

	pub fn empty() -> Mesh {
		Mesh {
			n_verts: 0,
			n_tris: 0,
			render_state: MeshRenderState::Unloaded {
				verts: Vec::new(),
				indices: Vec::new(),
				pos: [0.0, 0.0, 0.0],
				texture: TextureAtlasKey::Null,
			},
		}
	}
}
