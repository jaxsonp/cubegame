use cgmath::prelude::*;


/// Translates from opengl coord system to wgpu
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
	1.0, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, 0.5, 0.5,
	0.0, 0.0, 0.0, 1.0,
);


pub struct Camera {
	/// position
	pub eye: cgmath::Point3<f32>,
	pub target: cgmath::Point3<f32>,
	pub up: cgmath::Vector3<f32>,
	/// Aspect ratio, width to height
	pub aspect: f32,
	/// Y field of view
	pub fovy: f32,
	/// Near z value for clipping
	pub near_z: f32,
	/// Far z value for clipping
	pub far_z: f32,
}

impl Camera {
	pub fn view_proj_matrix(&self) -> [[f32; 4]; 4] {
		let view_mat = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
		let proj_mat = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.near_z, self.far_z);
		(OPENGL_TO_WGPU_MATRIX * proj_mat * view_mat).into()
	}
}