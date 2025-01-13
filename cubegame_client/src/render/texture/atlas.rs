// put this in its own file cus can, sue me

use std::collections::HashMap;

use super::LoadedTexture;
use crunch::*;
use cubegame_lib::blocks::NULL_BLOCK_ID;
use cubegame_lib::BlockTypeID;
use image::{imageops, RgbaImage};
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
		images: Vec<(TextureAtlasKey, RgbaImage)>,
		device: &wgpu::Device,
		queue: &wgpu::Queue,
	) -> Result<TextureAtlas, ()> {
		let max_size = device.limits().max_texture_dimension_2d as usize;
		let mut img_map: HashMap<TextureAtlasKey, &RgbaImage> = HashMap::new();

		// packing
		let items = images.iter().map(|(key, img)| -> Item<TextureAtlasKey> {
			img_map.insert(*key, img);
			Item::new(
				*key,
				img.width() as usize,
				img.height() as usize,
				Rotation::None,
			)
		});
		let result = match pack_into_po2(max_size, items) {
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
		for item in result.items.iter() {
			// overlay image onto atlas
			let img = *img_map.get(&item.data).unwrap();
			imageops::overlay(&mut atlas, img, item.rect.x as i64, item.rect.y as i64);

			// remember position on the atlas
			rect_map.insert(
				item.data,
				[
					item.rect.x as f32 / atlas.width() as f32,
					item.rect.y as f32 / atlas.height() as f32,
					item.rect.w as f32 / atlas.width() as f32,
					item.rect.h as f32 / atlas.height() as f32,
				],
			);
			if item.data == TextureAtlasKey::Block(NULL_BLOCK_ID) {
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

		//atlas.save("atlas.png").unwrap();
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
	Block(BlockTypeID),
	Null,
}
