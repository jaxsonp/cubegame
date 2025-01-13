// put this in its own file cus can, sue me

use super::LoadedTexture;
use crunch::*;
use cubegame_lib::blocks::NULL_BLOCK_ID;
use cubegame_lib::{BlockTypeId, Direction};
use image::{imageops, RgbaImage};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use wgpu::BindGroupLayoutDescriptor;

/// Wrapper struct for making and reading from a texture atlas
pub struct TextureAtlas {
	/// Texture containing all the sprites
	pub texture: LoadedTexture,
	/// Maps each key to a position on the atlas: [x pos, y pos, x scale, y scale]
	map: HashMap<TextureAtlasKey, [f32; 4]>,
}
impl TextureAtlas {
	pub fn generate(
		images: Vec<(Vec<TextureAtlasKey>, RgbaImage)>,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Result<TextureAtlas, ()> {
		let max_size = device.limits().max_texture_dimension_2d as usize;

		let get_hash_of_keys = |keys: &Vec<TextureAtlasKey>| -> u64 {
			let mut hasher = DefaultHasher::new();
			keys.hash(&mut hasher);
			hasher.finish()
		};

		// packing
		let items = images.iter().map(|(keys, img)| -> Item<u64> {
			Item::new(
				get_hash_of_keys(keys),
				img.width() as usize,
				img.height() as usize,
				Rotation::None,
			)
		});
		let result: PackedItems<u64> = match pack_into_po2(max_size, items) {
			Ok(res) => res,
			Err(_) => {
				log::error!("Failed to pack textures into texture atlas");
				return Err(());
			}
		};
		log::debug!("Packed {} textures into atlas", images.len());

		// creating image and position map
		let mut rect_map = HashMap::new();
		let mut atlas = RgbaImage::new(result.w as u32, result.h as u32);
		for item in result.items.into_iter() {
			// finding keys for this item by hash
			let keys_hash = item.data;
			let (keys, img) = images
				.iter()
				.find(|(keys, _img)| keys_hash == get_hash_of_keys(keys))
				.unwrap();

			// overlay image onto atlas
			imageops::overlay(&mut atlas, img, item.rect.x as i64, item.rect.y as i64);

			let rect = [
				item.rect.x as f32 / atlas.width() as f32,
				item.rect.y as f32 / atlas.height() as f32,
				item.rect.w as f32 / atlas.width() as f32,
				item.rect.h as f32 / atlas.height() as f32,
			];
			for key in keys {
				rect_map.insert(*key, rect);

				// adding special null key to null block
				if let TextureAtlasKey::Block(type_id) = key {
					if *type_id == NULL_BLOCK_ID {
						rect_map.insert(
							TextureAtlasKey::Null,
							[
								item.rect.x as f32 / atlas.width() as f32,
								item.rect.y as f32 / atlas.height() as f32,
								item.rect.w as f32 / atlas.width() as f32,
								item.rect.h as f32 / atlas.height() as f32,
							],
						);
					}
				}
			}
		}

		#[cfg(debug_assertions)]
		atlas.save("texture_atlas.png").unwrap();

		// loading
		let texture = LoadedTexture::load_from_img(
			atlas,
			"Texture atlas",
			device,
			queue,
			&device.create_bind_group_layout(&BindGroupLayoutDescriptor {
				label: Some("Texture atlas bind group layout"),
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
			}),
		);

		Ok(TextureAtlas {
			texture,
			map: rect_map,
		})
	}

	pub fn get_pos_of(&self, key: TextureAtlasKey) -> Option<&[f32; 4]> {
		self.map.get(&key)
	}
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum TextureAtlasKey {
	/// Sampling a uniform block
	Block(BlockTypeId),
	/// Sampling a specific face of a non-uniform block
	BlockFace(BlockTypeId, Direction),
	/*/// Sampling the side of a non-uniform block
	BlockSides(BlockTypeId),*/
	/// Null (fallback) texture
	Null,
}
