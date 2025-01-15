use super::BlockTypeId;
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone)]
pub struct BlockType {
	pub id: BlockTypeId,
	pub name: &'static str,
	pub texture_layout: BlockTextureLayout,
}
impl BlockType {
	pub fn is_air(&self) -> bool {
		self.id == AIR_BLOCK_ID
	}

	pub fn from_id(id: BlockTypeId) -> &'static BlockType {
		for t in BLOCK_TYPES.iter() {
			if t.id == id {
				return t;
			}
		}
		return &BLOCK_TYPES[NULL_BLOCK_ID as usize];
	}
}

#[derive(Debug, Copy, Clone)]
pub enum BlockTextureLayout {
	/// All faces have the same texture
	Uniform(&'static str),
	/// Different textures for the top, the sides, and the bottom
	TopSideBottom {
		top: &'static str,
		sides: &'static str,
		bottom: &'static str,
	},
	/// Has no textured faces (air)
	None,
}

pub static AIR_BLOCK_ID: BlockTypeId = 1;
pub static NULL_BLOCK_ID: BlockTypeId = 0;

// TODO add better registering functionality
// TODO add randomized textures
lazy_static! {
	/// Static list off all block types, needs to be initialized
	pub static ref BLOCK_TYPES: Vec<BlockType> = vec![
		BlockType {
			id: 0,
			name: "null_block",
			texture_layout: BlockTextureLayout::Uniform("null_block.png"),
		},
		BlockType {
			id: 1,
			name: "air",
			texture_layout: BlockTextureLayout::None,
		},
		BlockType {
			id: 2,
			name: "stone_block",
			texture_layout: BlockTextureLayout::Uniform("stone_block.png"),
		},
		BlockType {
			id: 3,
			name: "dirt_block",
			texture_layout: BlockTextureLayout::Uniform("dirt_block.png"),
		},
		BlockType {
			id: 4,
			name: "grass_block",
			texture_layout: BlockTextureLayout::TopSideBottom {
				top: "grass_block_top.png",
				sides: "grass_block_side.png",
				bottom: "dirt_block.png",
			},
		},
	];
}