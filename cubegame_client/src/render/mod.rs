pub mod mesh;
mod perspective;
mod pipelines;
mod texture;

use std::sync::Arc;

use pollster::FutureExt;
use winit::window::Window;

use crate::game::Game;
use perspective::Perspective;
use perspective::OPENGL_TO_WGPU_MATRIX;
use pipelines::{RenderPassInterface, WorldRenderingPipeline};
use texture::depth_buffer::DepthTexture;

pub struct Renderer {
	/// winit window needs to be an Arc because both this, the application, and the surface constructor (async) needs it
	pub window: Arc<Window>,
	/// Rendering surface
	surface: wgpu::Surface<'static>,
	/// Surface config if i had to guess
	surface_config: wgpu::SurfaceConfiguration,
	/// Connection to GPU
	pub device: wgpu::Device,
	/// Device command queue
	queue: wgpu::Queue,
	// Rendering pipelines
	/// World Rendering Pipeline, renders blocks and stuff
	world_render_pipeline: WorldRenderingPipeline,
	/// Layout of per-mesh bind group
	//pub mesh_bind_group_layout: wgpu::BindGroupLayout,
	/// Depth buffer texture (z buffer)
	depth_buffer: DepthTexture,
	/// Perspective matrix data used for 3d rendering
	pub perspective: Perspective,
	/// Buffer for camera data to go in
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
						required_features: wgpu::Features::POLYGON_MODE_LINE,
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
		/*let camera_bind_group_layout =
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
				let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
					layout: &camera_bind_group_layout,
					entries: &[wgpu::BindGroupEntry {
						binding: 0,
						resource: camera_buffer.as_entire_binding(),
					}],
					label: Some("Global bind group"),
				});
		*/
		let world_render_pipeline = WorldRenderingPipeline::new(
			&device,
			&queue,
			&config,
			camera_buffer.as_entire_binding(),
		)?;

		// loading block textures
		/*let texture_bind_group_layout =
		device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Texture {
						multisampled: false,
						view_dimension: wgpu::TextureViewDimension::D2,
						sample_type: wgpu::TextureSampleType::Float { filterable: false },
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
					count: None,
				},
			],
			label: Some("Texture bind group layout"),
		});*/

		let depth_texture = DepthTexture::new(&device, &config);

		Ok(Self {
			surface,
			device,
			queue,
			surface_config: config,
			window,
			world_render_pipeline,
			depth_buffer: depth_texture,
			perspective: Perspective::new(
				size.width as f32 / size.height as f32,
				70.0,
				0.1,
				1000.0,
			),
			camera_buffer,
		})
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.surface_config.width = new_size.width;
			self.surface_config.height = new_size.height;

			// updating camera aspect ratio
			self.perspective.aspect = new_size.width as f32 / new_size.height as f32;

			// resizing depth texture
			self.depth_buffer = DepthTexture::new(&self.device, &self.surface_config);

			self.reconfigure_surface()
		}
	}

	pub fn reconfigure_surface(&mut self) {
		self.surface.configure(&self.device, &self.surface_config);
	}

	/// Render the in game scene
	pub fn render_game(&mut self, game: &Game) -> Result<(), wgpu::SurfaceError> {
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

		// updating camera buffer
		let player_view_mat = game.world_data.player.view_matrix();

		let view_proj_matrix: [[f32; 4]; 4] =
			(OPENGL_TO_WGPU_MATRIX * self.perspective.proj_matrix() * player_view_mat).into();
		self.queue.write_buffer(
			&self.camera_buffer,
			0,
			bytemuck::cast_slice(&view_proj_matrix),
		);

		if self
			.world_render_pipeline
			.execute_render_pass(
				&mut encoder,
				&view,
				&self.depth_buffer.texture_view,
				&game.world_data,
			)
			.is_err()
		{
			log::error!("Failed to execute world render pass");
		}

		self.queue.submit(std::iter::once(encoder.finish()));

		// winit docs says to do this
		self.window.pre_present_notify();

		output.present();
		Ok(())
	}
}
