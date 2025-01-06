#![allow(unused)]

mod mesh;
mod camera;

use std::sync::Arc;
use winit::{
	application::ApplicationHandler,
	dpi::LogicalSize,
	event::*,
	event_loop::{ActiveEventLoop, EventLoop},
	keyboard::{KeyCode, PhysicalKey},
	window::{Window, WindowAttributes, WindowId},
};
use pollster::FutureExt as _;
use wgpu::util::DeviceExt;
use winit::event_loop::ControlFlow;


use mesh::{Mesh, vert::Vert};
use camera::Camera;

pub struct Renderer {
	surface: wgpu::Surface<'static>,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	pub window: Arc<Window>,
	render_pipeline: wgpu::RenderPipeline,
	camera: Camera,
	vertex_buffer: wgpu::Buffer,
	index_buffer: wgpu::Buffer,
	camera_buffer: wgpu::Buffer,
	camera_bind_group: wgpu::BindGroup,
}

impl Renderer {
	pub fn new(window: Window) -> Renderer {
		// window needs to be an Arc because both this struct and the surface constructor needs it
		let window = Arc::new(window);

		// async block cus some of the GPU adapter stuff is async
		return async move {
			let size = window.inner_size();

			let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
				backends: wgpu::Backends::PRIMARY,
				..Default::default()
			});

			// drawing target texture
			let surface = instance.create_surface(window.clone()).unwrap();

			// handle to GPU
			let adapter = instance
				.request_adapter(&wgpu::RequestAdapterOptions {
					power_preference: wgpu::PowerPreference::default(),
					compatible_surface: Some(&surface),
					force_fallback_adapter: false,
				})
				.await
				.unwrap();

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
			let surface_capabilities = surface.get_capabilities(&adapter);

			// we doing sRGB
			let surface_format = surface_capabilities
				.formats
				.iter()
				.find(|f| f.is_srgb())
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


			let camera = Camera {
				// position the camera 1 unit up and 2 units back
				// +z is out of the screen
				eye: (0.0, 1.0, 2.0).into(),
				// make it look at the origin
				target: (0.0, 0.0, 0.0).into(),
				// which way is "up"
				up: cgmath::Vector3::unit_y(),
				aspect: config.width as f32 / config.height as f32,
				fovy: 45.0,
				near_z: 0.1,
				far_z: 1000.0,
			};
			let camera_buffer = device.create_buffer_init(
				&wgpu::util::BufferInitDescriptor {
					label: Some("Camera Buffer"),
					contents: bytemuck::cast_slice(&[camera.view_proj_matrix()]),
					usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
				}
			);
			let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
					}
				],
				label: Some("camera_bind_group_layout"),
			});
			let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
				layout: &camera_bind_group_layout,
				entries: &[
					wgpu::BindGroupEntry {
						binding: 0,
						resource: camera_buffer.as_entire_binding(),
					}
				],
				label: Some("camera_bind_group"),
			});

			// setting up render pipeline
			let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
			let render_pipeline_layout =
				device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
					label: Some("Render Pipeline Layout"),
					bind_group_layouts: &[
						&camera_bind_group_layout,
					],
					push_constant_ranges: &[],
				});
			let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
				label: Some("Render Pipeline"),
				layout: Some(&render_pipeline_layout),
				vertex: wgpu::VertexState {
					module: &shader,
					entry_point: "vs_main",
					buffers: &[
						Vert::desc(), // vert buffer
					],
					compilation_options: wgpu::PipelineCompilationOptions::default(),
				},
				fragment: Some(wgpu::FragmentState {
					module: &shader,
					entry_point: "fs_main",
					targets: &[Some(wgpu::ColorTargetState {
						format: config.format,
						blend: Some(wgpu::BlendState::REPLACE),
						write_mask: wgpu::ColorWrites::ALL,
					})],
					compilation_options: wgpu::PipelineCompilationOptions::default(),
				}),
				primitive: wgpu::PrimitiveState {
					topology: wgpu::PrimitiveTopology::TriangleList, // 1.
					strip_index_format: None,
					front_face: wgpu::FrontFace::Ccw, // front face is counter-clockwise
					cull_mode: Some(wgpu::Face::Back), // back cull
					polygon_mode: wgpu::PolygonMode::Fill,
					// Requires Features::DEPTH_CLIP_CONTROL
					unclipped_depth: false,
					// Requires Features::CONSERVATIVE_RASTERIZATION
					conservative: false,
				},
				depth_stencil: None, // 1.
				multisample: wgpu::MultisampleState { // idk about this stuff
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				},
				multiview: None, // 5.
				cache: None, // 6.
			});

			// generating test mesh
			let cube = Mesh::cube();
			let vertex_buffer = device.create_buffer_init(
				&wgpu::util::BufferInitDescriptor {
					label: Some("Vertex Buffer"),
					contents: bytemuck::cast_slice(cube.verts),
					usage: wgpu::BufferUsages::VERTEX,
				}
			);
			let index_buffer = device.create_buffer_init(
				&wgpu::util::BufferInitDescriptor {
					label: Some("Index Buffer"),
					contents: bytemuck::cast_slice(cube.tris),
					usage: wgpu::BufferUsages::INDEX,
				}
			);



			log::debug!("Instantiated renderer");
			Self {
				surface,
				device,
				queue,
				config,
				size,
				window,
				render_pipeline,
				camera,
				vertex_buffer,
				index_buffer,
				camera_buffer,
				camera_bind_group,
			}
		}.block_on();
	}

	pub fn handle_event(&mut self, event: WindowEvent, event_loop: &ActiveEventLoop) {
		match event {
			WindowEvent::RedrawRequested => {
				self.window.request_redraw();

				self.update();
				match self.render() {
					Ok(_) => {}
					Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
						// Reconfigure the surface if it's lost or outdated
						self.resize(self.size)
					}
					Err(wgpu::SurfaceError::OutOfMemory) => {
						log::error!("OutOfMemory");
						event_loop.exit();
					}
					Err(wgpu::SurfaceError::Timeout) => {
						log::warn!("Surface timeout")
					}
				}
			}
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::Resized(physical_size) => {
				self.resize(physical_size);
			}

			_ => {}
		}
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	fn update(&mut self) {
		//todo!()
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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

		// creating render pass
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color {
						r: 0.1,
						g: 0.2,
						b: 0.3,
						a: 1.0,
					}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});
		render_pass.set_pipeline(&self.render_pipeline);
		render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

		// giving buffers to pipeline
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

		render_pass.draw_indexed(0..36, 0, 0..1);
		drop(render_pass);

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

