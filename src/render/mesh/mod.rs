
pub mod vert;

use vert::Vert;

pub struct Mesh<'a> {
	pub n_verts: usize,
	pub verts: &'a[Vert],
	pub n_tris: usize,
	pub tris: &'a[u32]
}
impl<'a> Mesh<'a> {
	/// Generates a cube mesh for testing
	pub fn cube() -> Mesh<'a> {
		Self {
			n_verts: 8,
			verts: &[
				Vert { pos: [-0.5, -0.5, -0.5] },
				Vert { pos: [0.5, -0.5, -0.5] },
				Vert { pos: [0.5, -0.5, 0.5] },
				Vert { pos: [-0.5, -0.5, 0.5] },
				Vert { pos: [-0.5, 0.5, -0.5] },
				Vert { pos: [0.5, 0.5, -0.5] },
				Vert { pos: [0.5, 0.5, 0.5] },
				Vert { pos: [-0.5, 0.5, 0.5] },
			],
			n_tris: 12,
			tris: &[
				0, 1, 2,
				0, 2, 3,
				0, 4, 5,
				0, 5, 1,
				1, 5, 6,
				1, 6, 2,
				2, 6, 7,
				2, 7, 3,
				3, 7, 4,
				3, 4, 0,
				4, 6, 5,
				4, 7, 6
			],
		}
	}
}