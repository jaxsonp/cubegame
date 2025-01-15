mod world_rendering;

use wgpu::{CommandEncoder, TextureView};
pub use world_rendering::WorldRenderingPipeline;

/// Describes a render pass
pub trait RenderPassInterface<ArgType> {
	fn execute_render_pass(
		&self,
		encoder: &mut CommandEncoder,
		surface_texture_view: &TextureView,
		depth_texture_view: &TextureView,
		argument: &ArgType,
	) -> Result<(), ()>;
}
