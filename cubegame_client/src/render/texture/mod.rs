pub mod atlas;
pub mod depth_buffer;

use image::RgbaImage;
use wgpu::{
	BindGroupDescriptor, BindGroupLayout, Device, Queue, Sampler, Texture, TextureDescriptor,
	TextureView,
};

/// Represents a loaded texture, ready for use with a render pass
#[allow(dead_code)]
pub struct LoadedTexture {
	texture: Texture,
	pub texture_view: TextureView,
	pub sampler: Sampler,
	pub bind_group: wgpu::BindGroup,
}
impl LoadedTexture {
	pub fn load_from_img(
		img: RgbaImage,
		name: &str,
		device: &Device,
		queue: &Queue,
		bind_group_layout: &BindGroupLayout,
	) -> LoadedTexture {
		let dimensions = img.dimensions();

		let texture_size = wgpu::Extent3d {
			width: dimensions.0,
			height: dimensions.1,
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&TextureDescriptor {
			label: Some(name),
			size: texture_size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::Rgba8UnormSrgb, // <-- format for srgb ig
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
			view_formats: &[],
		});

		// writing image data to texture
		queue.write_texture(
			wgpu::ImageCopyTexture {
				texture: &texture,
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
				aspect: wgpu::TextureAspect::All,
			},
			&img,
			wgpu::ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(4 * dimensions.0),
				rows_per_image: Some(dimensions.1),
			},
			texture_size,
		);

		let texture_view = texture.create_view(&Default::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			..Default::default()
		});
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			layout: bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&texture_view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&sampler),
				},
			],
			label: Some((name.to_string() + " texture bind group").as_str()),
		});

		log::debug!(
			"Loaded texture for \"{}\" ({}x{})",
			name,
			dimensions.0,
			dimensions.1
		);
		LoadedTexture {
			texture,
			texture_view,
			sampler,
			bind_group,
		}
	}
}
