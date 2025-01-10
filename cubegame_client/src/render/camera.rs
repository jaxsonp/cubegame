use crate::game::player::Player;
use nalgebra::{geometry::Perspective3, Matrix4, Point3, Vector3};

/// Translates from opengl coord system to wgpu
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
	1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
	/// position
	pub eye: Point3<f32>,
	pub facing: Vector3<f32>,
	pub up: Vector3<f32>,
	/// Aspect ratio, width to height
	pub aspect: f32,
	/// Y field of view (in degrees)
	pub fovy: f32,
	/// Near z value for clipping
	pub near_z: f32,
	/// Far z value for clipping
	pub far_z: f32,
}

impl Camera {
	pub fn new(aspect: f32, fovy: f32, near_z: f32, far_z: f32) -> Camera {
		Camera {
			eye: Point3::new(0.0, 0.0, 0.0),
			facing: Vector3::zeros(),
			up: Vector3::new(0.0, 1.0, 0.0),
			aspect,
			fovy,
			near_z,
			far_z,
		}
	}

	pub fn view_proj_matrix(&self) -> [[f32; 4]; 4] {
		let target = self.eye + self.facing;
		let view_mat = Matrix4::look_at_rh(&self.eye, &target, &self.up);
		let proj_mat =
			Perspective3::new(self.aspect, self.fovy.to_radians(), self.near_z, self.far_z)
				.to_homogeneous();
		(OPENGL_TO_WGPU_MATRIX * proj_mat * view_mat).into()
	}

	pub fn player_pov(&mut self, player: &Player) {
		self.eye = player.pos;
		self.facing = player.facing_vec();
		self.up = Vector3::new(0.0, 1.0, 0.0);
	}
}
