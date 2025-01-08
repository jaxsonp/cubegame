use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone)]
pub struct BlockType {
	pub id: u8,
	pub name: &'static str,
	pub texture_path: &'static str,
}
impl BlockType {
	pub fn is_air(&self) -> bool {
		self.id == AIR_BLOCK_ID
	}
}

pub static AIR_BLOCK_ID: u8 = 0;
pub static NULL_BLOCK_ID: u8 = 1;

// TODO add better registering functionality
lazy_static! {
	/// Static list off all block types, needs to be initialized
	pub static ref BLOCK_TYPES: Vec<BlockType> = vec![
		BlockType {
			id: 0,
			name: "air",
			texture_path: "",
		},
		BlockType {
			id: 1,
			name: "null_block",
			texture_path: "block_textures/null_block.png",
		},
		BlockType {
			id: 2,
			name: "stone_block",
			texture_path: "block_textures/stone_block.png",
		},
	];
}


