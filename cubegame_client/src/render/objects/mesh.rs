use crate::render::objects::vert::MeshVert;
use crate::render::texture::atlas::TextureAtlasKey;
use crate::render::Renderer;
use wgpu::{
	util::{BufferInitDescriptor, DeviceExt},
	BufferUsages,
};

/// Represents whether the objects has been loaded into buffers and such, and if not, contains the data to load it
pub enum MeshRenderState {
	Loaded(MeshRenderObjects),
	Unloaded {
		verts: Vec<MeshVert>,
		indices: Vec<u32>,
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
	/// Position offset
	pub pos: [f32; 3],
	/// Whether this objects has been loaded
	pub render_state: MeshRenderState,
}
impl Mesh {
	pub fn new(
		verts: Vec<MeshVert>,
		indices: Vec<u32>,
		pos: &[f32; 3],
		texture: TextureAtlasKey,
	) -> Mesh {
		Mesh {
			n_verts: verts.len() as u32,
			n_tris: indices.len() as u32 / 3,
			pos: *pos,
			render_state: MeshRenderState::Unloaded {
				verts,
				indices,
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

	/// Creates buffers and bind group for this objects if it hasn't been loaded already
	pub fn load_buffers(&mut self, renderer: &Renderer) {
		match &self.render_state {
			MeshRenderState::Unloaded {
				verts,
				indices,
				texture,
			} => {
				// position offset of this whole objects
				let pos_buffer = renderer.device.create_buffer_init(&BufferInitDescriptor {
					label: Some("Mesh position buffer"),
					contents: bytemuck::cast_slice(&self.pos),
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

	/// Adds one objects into another (both must be unloaded
	/*pub fn union(&mut self, new_mesh: Mesh) -> Result<(), ()> {
		match (&mut self.render_state, new_mesh.render_state) {
			(
				MeshRenderState::Unloaded {
					verts,
					indices,
					texture,
				},
				MeshRenderState::Unloaded {
					verts: new_verts,
					indices: new_indices,
					texture: _,
				},
			) => {
				// updating verts
				verts.extend(new_verts.iter().map(|vert| {
					let mut new_vert = *vert;
					for i in 0..3 {
						new_vert.pos[i] += new_mesh.pos[i];
						new_vert.pos[i] -= self.pos[i];
					}
					new_vert
				}));
				self.n_verts += new_mesh.n_verts;

				// updating indices
				indices.extend(new_indices.iter().map(|i| i + self.n_verts));
				self.n_tris += new_mesh.n_tris;

				Ok(())
			}
			_ => {
				log::warn!("Tried to union loaded meshes");
				return Err(());
			}
		}
	}*/

	/// Generates a cube objects with only certain faces
	/*pub fn new_block_from_faces(
		faces: Direction,
		x: i32,
		y: i32,
		z: i32,
		block_type_id: BlockTypeID,
	) -> Mesh {
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

		Self {
			pos: [x as f32, y as f32, z as f32],
			n_verts,
			n_tris,
			render_state: MeshRenderState::Unloaded {
				verts,
				indices,
				texture: TextureAtlasKey::Block(block_type_id),
			},
		}
	}*/

	pub fn empty() -> Mesh {
		Mesh {
			pos: [0.0, 0.0, 0.0],
			n_verts: 0,
			n_tris: 0,
			render_state: MeshRenderState::Unloaded {
				verts: Vec::new(),
				indices: Vec::new(),
				texture: TextureAtlasKey::Null,
			},
		}
	}
}
