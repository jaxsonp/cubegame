use wgpu::{
	Device, SurfaceConfiguration, Texture, TextureDescriptor, TextureFormat, TextureView,
	TextureViewDescriptor,
};

/// Struct to hold depth buffer, has same size as screen
#[allow(dead_code)]
pub struct DepthTexture {
	pub texture: Texture,
	pub texture_view: TextureView,
}

impl DepthTexture {
	pub const FORMAT: TextureFormat = TextureFormat::Depth32Float;

	pub fn new(device: &Device, config: &SurfaceConfiguration) -> DepthTexture {
		let texture_size = wgpu::Extent3d {
			width: config.width.max(1),
			height: config.height.max(1),
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&TextureDescriptor {
			label: Some("Depth buffer texture"),
			size: texture_size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: DepthTexture::FORMAT,
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		});

		let texture_view = texture.create_view(&TextureViewDescriptor::default());
		DepthTexture {
			texture,
			texture_view,
		}
	}
}
