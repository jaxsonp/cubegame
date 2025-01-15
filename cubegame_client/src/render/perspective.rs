use nalgebra::{geometry::Perspective3, Matrix4};

/// Translates from opengl coord system to wgpu
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
	1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

pub struct Perspective {
	/// Aspect ratio, width to height
	pub aspect: f32,
	/// Y field of view (in degrees)
	pub fovy: f32,
	/// Near z value for clipping
	pub near_z: f32,
	/// Far z value for clipping
	pub far_z: f32,
}

impl Perspective {
	pub fn new(aspect: f32, fovy: f32, near_z: f32, far_z: f32) -> Perspective {
		Perspective {
			aspect,
			fovy,
			near_z,
			far_z,
		}
	}

	pub fn proj_matrix(&self) -> Matrix4<f32> {
		Perspective3::new(self.aspect, self.fovy.to_radians(), self.near_z, self.far_z)
			.to_homogeneous()
	}
}
