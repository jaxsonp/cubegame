use std::collections::HashMap;
use std::path::{Path, PathBuf};

use cubegame_lib::blocks::{BlockTextureLayout, BLOCK_TYPES};
use cubegame_lib::Direction;
use image::{ImageReader, RgbaImage};

use super::RenderPassInterface;
use crate::{
	game::world::WorldData,
	render::{
		mesh::vert::Vert,
		texture::{
			atlas::{TextureAtlas, TextureAtlasKey},
			depth_buffer::DepthTexture,
		},
	},
};

/// Render pipeline for rendering the world (blocks and stuff)
///
/// Bind groups and bindings:
/// 	0: "global" set once per frame
/// 		0 - Camera (view/projection) matrix
/// 		1 - Texture atlas texture view
/// 		2 - Texture atlas sampler
/// 	1: "local" set once per mesh/object
pub struct WorldRenderingPipeline {
	pipeline: wgpu::RenderPipeline,
	global_bind_group: wgpu::BindGroup,
	/// Layout of the local bind group for each mesh
	pub(crate) local_bind_group_layout: wgpu::BindGroupLayout,
	/// Atlas containing all the packed block textures
	pub block_texture_atlas: TextureAtlas,
}
impl WorldRenderingPipeline {
	pub fn new(
		device: &wgpu::Device,
		queue: &wgpu::Queue,
		surface_config: &wgpu::SurfaceConfiguration,
		camera_bind_resource: wgpu::BindingResource,
	) -> Result<Self, ()> {
		let block_texture_atlas = TextureAtlas::generate(read_block_textures()?, &device, &queue)?;

		let global_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::VERTEX,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Texture {
							multisampled: false,
							view_dimension: wgpu::TextureViewDimension::D2,
							sample_type: wgpu::TextureSampleType::Float { filterable: false },
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 2,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
						count: None,
					},
				],
				label: Some("World rendering global bind group layout"),
			});
		let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &global_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: camera_bind_resource,
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::TextureView(
						&block_texture_atlas.texture.texture_view,
					),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: wgpu::BindingResource::Sampler(&block_texture_atlas.texture.sampler),
				},
			],
			label: Some("World rendering global bind group"),
		});
		let local_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::VERTEX,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
				],
				label: Some("World rendering local bind group layout"),
			});
		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("World rendering pipeline layout"),
			bind_group_layouts: &[&global_bind_group_layout, &local_bind_group_layout],
			push_constant_ranges: &[],
		});
		let vert_shader =
			device.create_shader_module(wgpu::include_wgsl!("world_shader_vert.wgsl"));
		let frag_shader =
			device.create_shader_module(wgpu::include_wgsl!("world_shader_frag.wgsl"));
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("World rendering pipeline"),
			layout: Some(&layout),
			vertex: wgpu::VertexState {
				module: &vert_shader,
				entry_point: "main",
				buffers: &[
					Vert::buffer_layout(), // vert buffer
				],
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &frag_shader,
				entry_point: "main",
				targets: &[Some(wgpu::ColorTargetState {
					format: surface_config.format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})],
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw, // front face is counter-clockwise
				cull_mode: Some(wgpu::Face::Back), // back cull
				polygon_mode: wgpu::PolygonMode::Fill,
				// Requires Features::DEPTH_CLIP_CONTROL
				unclipped_depth: false,
				// Requires Features::CONSERVATIVE_RASTERIZATION
				conservative: false,
			},
			depth_stencil: Some(wgpu::DepthStencilState {
				format: DepthTexture::FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState {
				// idek what this stuff does
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
			cache: None,
		});

		Ok(WorldRenderingPipeline {
			pipeline,
			global_bind_group,
			local_bind_group_layout,
			block_texture_atlas,
		})
	}
}
impl RenderPassInterface<WorldData> for WorldRenderingPipeline {
	/// Clears to white, also clears depth texture
	fn execute_render_pass(
		&self,
		encoder: &mut wgpu::CommandEncoder,
		surface_texture_view: &wgpu::TextureView,
		depth_texture_view: &wgpu::TextureView,
		world_data: &WorldData,
	) -> Result<(), ()> {
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("World rendering pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &surface_texture_view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
				view: depth_texture_view,
				depth_ops: Some(wgpu::Operations {
					load: wgpu::LoadOp::Clear(1.0),
					store: wgpu::StoreOp::Store,
				}),
				stencil_ops: None,
			}),
			occlusion_query_set: None,
			timestamp_writes: None,
		});
		render_pass.set_pipeline(&self.pipeline);

		// setting global bind group
		render_pass.set_bind_group(0, &self.global_bind_group, &[]);

		// drawing chunks
		for (_pos, chunk) in world_data.chunks.iter() {
			for mesh in chunk.meshes.iter() {
				let Some(mesh_render_objs) = mesh.get_render_objs() else {
					continue;
				};

				// setting local bind group
				render_pass.set_bind_group(1, &mesh_render_objs.bind_group, &[]);

				// setting vert and tri buffers
				render_pass.set_vertex_buffer(0, mesh_render_objs.vertex_buffer.slice(..));
				render_pass.set_index_buffer(
					mesh_render_objs.index_buffer.slice(..),
					wgpu::IndexFormat::Uint32,
				);

				// draw
				render_pass.draw_indexed(0..(mesh.n_tris * 3), 0, 0..1);
			}
		}
		Ok(())
	}
}

/// helper function that reads block textures for every block type from file
///
/// returns a vector of keys that belong to an image
pub fn read_block_textures() -> Result<Vec<(Vec<TextureAtlasKey>, RgbaImage)>, ()> {
	// hashmap of every image path and the keys that need it.
	// This exists to remove redundant image loads, if there are multiple block types that reference the same file
	let mut filepaths: HashMap<PathBuf, Vec<TextureAtlasKey>> = HashMap::new();

	let mut record_filepath = |path: PathBuf, keys: Vec<TextureAtlasKey>| {
		if filepaths.contains_key(&path) {
			filepaths.get_mut(&path).unwrap().extend(keys);
		} else {
			filepaths.insert(path, keys);
		}
	};

	for block_type in BLOCK_TYPES.iter() {
		match block_type.texture_layout {
			BlockTextureLayout::Uniform(filename) => {
				record_filepath(
					Path::new("./assets/block_textures").join(filename),
					vec![TextureAtlasKey::Block(block_type.id)],
				);
			}
			BlockTextureLayout::TopSideBottom {
				top: top_filename,
				sides: side_filename,
				bottom: bottom_filename,
			} => {
				record_filepath(
					Path::new("./assets/block_textures").join(top_filename),
					vec![TextureAtlasKey::BlockFace(block_type.id, Direction::PosY)],
				);
				record_filepath(
					Path::new("./assets/block_textures").join(side_filename),
					vec![
						TextureAtlasKey::BlockFace(block_type.id, Direction::PosX),
						TextureAtlasKey::BlockFace(block_type.id, Direction::NegX),
						TextureAtlasKey::BlockFace(block_type.id, Direction::PosZ),
						TextureAtlasKey::BlockFace(block_type.id, Direction::NegZ),
					],
				);
				record_filepath(
					Path::new("./assets/block_textures").join(bottom_filename),
					vec![TextureAtlasKey::BlockFace(block_type.id, Direction::NegY)],
				);
			}
			BlockTextureLayout::None => {}
		}
	}

	// reading the images
	let mut out = Vec::new();
	for (path, keys) in filepaths.into_iter() {
		let img = match ImageReader::open(&path) {
			Ok(img_reader) => img_reader.decode().unwrap().to_rgba8(),
			Err(e) => {
				log::error!(
					"Failed to read block texture from \"{}\": {}",
					path.display(),
					e
				);
				return Err(());
			}
		};
		out.push((keys, img));
	}

	return Ok(out);
}
