use super::super::objects::LineVert;
use crate::{
	game::world::WorldData, render::texture::depth_buffer::DepthTexture, CHUNK_BORDER_COLOR,
};
use cubegame_lib::{CHUNK_WIDTH, WORLD_HEIGHT};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

/// Render pipeline for rendering debug lines and stuff
///
/// Bind groups and bindings:
/// 	0: "global" set once per frame ( !!! SET BY WORLD RENDERING PASS EARLIER )
/// 		0 - Camera (view/projection) matrix: 4x4 float matrix
/// 		1 - Line color : float vector3
pub struct LineRenderingPipeline {
	pipeline: wgpu::RenderPipeline,
	global_bind_group: wgpu::BindGroup,
	pub show_chunk_borders: bool,
}
impl LineRenderingPipeline {
	pub fn new(
		device: &wgpu::Device,
		surface_config: &wgpu::SurfaceConfiguration,
		camera_bind_resource: wgpu::BindingResource,
	) -> Result<Self, ()> {
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
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
				],
				label: Some("Line rendering global bind group layout"),
			});
		let color_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Line rendering color buffer"),
			contents: &bytemuck::cast_slice(&CHUNK_BORDER_COLOR),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
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
					resource: color_buffer.as_entire_binding(),
				},
			],
			label: Some("Line rendering global bind group"),
		});
		let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Line rendering pipeline layout"),
			bind_group_layouts: &[&global_bind_group_layout],
			push_constant_ranges: &[],
		});
		let shader = device.create_shader_module(wgpu::include_wgsl!("line_shader.wgsl"));
		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Line rendering pipeline"),
			layout: Some(&layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: Some("vs_main"),
				buffers: &[
					LineVert::buffer_layout(), // vert buffer
				],
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: Some("fs_main"),
				targets: &[Some(wgpu::ColorTargetState {
					format: surface_config.format,
					blend: Some(wgpu::BlendState::REPLACE),
					write_mask: wgpu::ColorWrites::ALL,
				})],
				compilation_options: wgpu::PipelineCompilationOptions::default(),
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::LineList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				polygon_mode: wgpu::PolygonMode::Line,
				unclipped_depth: false,
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

		Ok(LineRenderingPipeline {
			pipeline,
			global_bind_group,
			show_chunk_borders: true,
		})
	}

	/// Executes a render pass on the given command encoder
	///
	/// Loads previous color attachment and depth attachment, discards depth texture when done
	pub fn execute_render_pass(
		&self,
		encoder: &mut wgpu::CommandEncoder,
		surface_texture_view: &wgpu::TextureView,
		depth_texture_view: &wgpu::TextureView,
		device: &wgpu::Device,
		world_data: &WorldData,
	) {
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Line rendering pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &surface_texture_view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Load,
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
				view: depth_texture_view,
				depth_ops: Some(wgpu::Operations {
					load: wgpu::LoadOp::Load,
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

		if self.show_chunk_borders {
			for (pos, _chunk) in world_data.chunks.iter() {
				let x = (pos.x * (CHUNK_WIDTH as i32)) as f32;
				let z = (pos.z * (CHUNK_WIDTH as i32)) as f32;
				let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: Some("Chunk border line vertex buffer"),
					contents: bytemuck::cast_slice(&[
						LineVert::new(x, 0.0, z),
						LineVert::new(x, WORLD_HEIGHT as f32, z),
					]),
					usage: wgpu::BufferUsages::VERTEX,
				});
				render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
				render_pass.draw(0..2, 0..1);
			}
		}
	}
}
