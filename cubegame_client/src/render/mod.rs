mod camera;
pub mod mesh;
mod texture;

use std::{collections::HashMap, path::Path, sync::Arc};

use image::ImageReader;
use pollster::FutureExt as _;
use winit::window::Window;

use crate::{game::Game, render::texture::DepthTexture};
use camera::Camera;
use cubegame_lib::blocks::*;
use mesh::vert::Vert;
use texture::LoadedTexture;

pub struct Renderer {
	/// winit window needs to be an Arc because both this, the application, and the surface constructor (async) needs it
	pub window: Arc<Window>,
	/// Rendering surface
	surface: wgpu::Surface<'static>,
	/// Surface config if i had to guess
	config: wgpu::SurfaceConfiguration,
	/// Connection to GPU
	device: wgpu::Device,
	/// Device command queue
	queue: wgpu::Queue,
	/// Render pipeline (only one for now)
	render_pipeline: wgpu::RenderPipeline,
	/// Bind group that is set once for all meshes
	global_bind_group: wgpu::BindGroup,
	/// Layout of per-mesh bind group
	mesh_bind_group_layout: wgpu::BindGroupLayout,
	/// Depth buffer texture (z buffer)
	depth_texture: DepthTexture,
	/// Loaded block textures
	block_textures: HashMap<u8, LoadedTexture>,
	/// Fallback block texture
	fallback_block_texture: LoadedTexture,
	/// Camera duh
	pub camera: Camera,
	camera_buffer: wgpu::Buffer,
}
impl Renderer {
	pub fn new(window: Arc<Window>) -> Result<Renderer, ()> {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			..Default::default()
		});

		let surface = instance.create_surface(window.clone()).unwrap();

		// getting some GPU adapter stuff is async
		let (adapter, device, queue) = async {
			// handle to GPU
			let adapter = instance
				.request_adapter(&wgpu::RequestAdapterOptions {
					power_preference: wgpu::PowerPreference::default(),
					compatible_surface: Some(&surface),
					force_fallback_adapter: false,
				})
				.await
				.unwrap();
			let adapter_info = adapter.get_info();
			log::debug!(
				"Adapter: {} ({:?})",
				adapter_info.name,
				adapter_info.backend
			);

			let (device, queue) = adapter
				.request_device(
					&wgpu::DeviceDescriptor {
						required_features: wgpu::Features::empty(),
						required_limits: wgpu::Limits::default(),
						label: None,
						// performance over efficiency for memory management
						memory_hints: wgpu::MemoryHints::Performance,
					},
					None,
				)
				.await
				.unwrap();
			(adapter, device, queue)
		}
			.block_on();

		let surface_capabilities = surface.get_capabilities(&adapter);

		let surface_format = surface_capabilities
			.formats
			.iter()
			.find(|f| f.is_srgb()) // we doing sRGB
			.copied()
			.unwrap_or(surface_capabilities.formats[0]);
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_capabilities.present_modes[0],
			alpha_mode: surface_capabilities.alpha_modes[0],
			view_formats: vec![],
			desired_maximum_frame_latency: 2,
		};
		surface.configure(&device, &config);

		// setting up global bind group
		let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("Camera Buffer"),
			size: size_of::<[[f32; 4]; 4]>() as u64,
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
			mapped_at_creation: false,
		});
		let global_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
				label: Some("Global bind group layout"),
			});
		let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &global_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: camera_buffer.as_entire_binding(),
			}],
			label: Some("Global bind group"),
		});
		let mesh_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
				label: Some("Local bind group layout"),
			});

		// loading block textures
		let texture_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Texture {
							multisampled: false,
							view_dimension: wgpu::TextureViewDimension::D2,
							sample_type: wgpu::TextureSampleType::Float { filterable: true },
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::FRAGMENT,
						// This should match the filterable field of the
						// corresponding Texture entry above.
						ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
						count: None,
					},
				],
				label: Some("Texture bind group layout"),
			});
		let mut block_textures: HashMap<u8, LoadedTexture> =
			HashMap::with_capacity(BLOCK_TYPES.len());
		for block in BLOCK_TYPES.iter() {
			if block.id == AIR_BLOCK_ID {
				continue; // no need to load a texture for air
			}

			// reading the image from file
			let path = Path::new("./assets/").join(block.texture_path);
			let img = match ImageReader::open(&path) {
				Ok(img_reader) => match img_reader.decode() {
					Ok(img) => img,
					Err(e) => {
						log::error!("Failed to decode asset \"{}\": {}", path.display(), e);
						return Err(());
					}
				},
				Err(e) => {
					log::error!(
						"Failed to load block texture for \"{}\" from \"{}\": {}",
						block.name,
						path.display(),
						e
					);
					return Err(());
				}
			};
			let loaded_texture = LoadedTexture::from_img(
				img.to_rgba8(),
				block.name,
				&device,
				&queue,
				&texture_bind_group_layout,
			);
			block_textures.insert(block.id, loaded_texture);
		}
		let fallback_block_texture = match block_textures.remove(&NULL_BLOCK_ID) {
			Some(tex) => tex,
			None => {
				log::error!("Fallback block texture ({NULL_BLOCK_ID}) not loaded");
				return Err(());
			}
		};

		// setting up render pipeline
		let vert_shader = device.create_shader_module(wgpu::include_wgsl!("shader_vert.wgsl"));
		let frag_shader = device.create_shader_module(wgpu::include_wgsl!("shader_frag.wgsl"));
		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				bind_group_layouts: &[
					&global_bind_group_layout,
					&texture_bind_group_layout,
					&mesh_bind_group_layout,
				],
				push_constant_ranges: &[],
			});
		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
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
					format: config.format,
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

		let depth_texture = DepthTexture::new(&device, &config);

		Ok(Self {
			surface,
			device,
			queue,
			config,
			window,
			render_pipeline,
			global_bind_group,
			mesh_bind_group_layout,
			depth_texture,
			block_textures,
			fallback_block_texture,
			camera: Camera::new(size.width as f32 / size.height as f32, 70.0, 0.1, 1000.0),
			camera_buffer,
		})
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.config.width = new_size.width;
			self.config.height = new_size.height;

			// updating camera aspect ratio
			self.camera.aspect = new_size.width as f32 / new_size.height as f32;

			// resizing depth texture
			self.depth_texture = DepthTexture::new(&self.device, &self.config);

			self.reconfigure()
		}
	}

	pub fn reconfigure(&mut self) {
		self.surface.configure(&self.device, &self.config);
	}

	pub fn render(&mut self, game: &Option<Game>) -> Result<(), wgpu::SurfaceError> {
		// updating uniforms
		self.queue.write_buffer(
			&self.camera_buffer,
			0,
			bytemuck::cast_slice(&self.camera.view_proj_matrix()),
		);

		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		// for talking w GPU
		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		if let Some(game) = game {
			// creating render pass
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_texture.texture_view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: wgpu::StoreOp::Store,
					}),
					stencil_ops: None,
				}),
				occlusion_query_set: None,
				timestamp_writes: None,
			});
			render_pass.set_pipeline(&self.render_pipeline);

			// global bind group
			render_pass.set_bind_group(0, &self.global_bind_group, &[]);

			for mesh in game.world.chunk.meshes.iter() {
				// TODO reduce redundant texture loads
				// activating texture
				let tex = &self.block_textures[&2];
				render_pass.set_bind_group(1, &tex.bind_group, &[]);

				// setting per-mesh bind group
				render_pass.set_bind_group(2, &mesh.bind_group, &[]);

				// setting verts and tris
				render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
				render_pass
					.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

				// draw
				render_pass.draw_indexed(0..(mesh.n_tris * 3), 0, 0..1);
			}

			// need to drop render pass before doing anything else w the encoder
		}

		// winit docs says to do this
		self.window.pre_present_notify();

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
