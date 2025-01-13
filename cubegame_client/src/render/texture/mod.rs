pub mod atlas;
pub mod depth_buffer;

use crate::render::texture::atlas::TextureAtlasKey;
use cubegame_lib::blocks::{BlockTextureLayout, BLOCK_TYPES};
use cubegame_lib::Direction;
use image::{ImageReader, RgbaImage};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use wgpu::{
	BindGroupDescriptor, BindGroupLayout, Device, Queue, Sampler, Texture, TextureDescriptor,
	TextureView,
};

/// Represents a loaded texture, ready for use with a render pass
#[allow(dead_code)]
pub struct LoadedTexture {
	texture: Texture,
	texture_view: TextureView,
	sampler: Sampler,
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
