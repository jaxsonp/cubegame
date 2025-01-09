pub mod controller;

use std::f32::consts::PI;

use nalgebra::{Point3, Vector3, Rotation3};

use controller::PlayerController;

pub struct Player {
	/// position
	pub pos: Point3<f32>,
	/// Yaw from negative Z (0 -> 2PI)
	pub facing_yaw: f32,
	/// pitch from horizon (-PI -> PI)
	pub facing_pitch: f32,
	/// Stores state on player input
	controller: PlayerController,
}
impl Player {
	// movement speeds, in units per second
	const MOVE_SPEED_FORWARD: f32 = 7.0;
	const MOVE_SPEED_BACKWARD: f32 = 6.0;
	const MOVE_SPEED_LATERAL: f32 = 6.0;
	const MOVE_SPEED_VERTICAL: f32 = 6.0;
	// look rotation speed, in degrees per second
	const LOOK_SPEED: f32 = 90.0;

	pub fn new() -> Self {
		Self {
			controller: PlayerController::new(),
			pos: Point3::new(0.0, 0.0, 2.0),
			facing_yaw: 0.0,
			facing_pitch: 0.0,
		}
	}

	pub fn handle_input(&mut self, event: &winit::event::WindowEvent) {
		// TODO make movement be handled server side
		self.controller.handle_input(event);
	}

	/// Unit vector representing direction player is facing
	pub fn facing_vec(&self) -> Vector3<f32> {
		let pitch_rot = Rotation3::from_axis_angle(&Vector3::x_axis(), self.facing_pitch);
		let yaw_rot = Rotation3::from_axis_angle(&Vector3::y_axis(), self.facing_yaw);
		yaw_rot * (pitch_rot * -Vector3::z())
	}

	pub fn update(&mut self, dt: f32) {
		let up = Vector3::<f32>::y();
		let left = up.cross(&self.facing_vec());
		let forward = left.cross(&up);

		if self.controller.forward() {
			self.pos += forward * Self::MOVE_SPEED_FORWARD * dt;
		} else if self.controller.backward() {
			self.pos += -forward * Self::MOVE_SPEED_BACKWARD * dt;
		}
		if self.controller.left() {
			self.pos += left * Self::MOVE_SPEED_LATERAL * dt;
		} else if self.controller.right() {
			self.pos += -left * Self::MOVE_SPEED_LATERAL * dt;
		}
		if self.controller.up() {
			self.pos += up * Self::MOVE_SPEED_VERTICAL * dt;
		} else if self.controller.down() {
			self.pos += -up * Self::MOVE_SPEED_VERTICAL * dt;
		}
		if self.controller.looking_up() {
			self.facing_pitch += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self.facing_pitch.clamp(-PI, PI);
		} else if self.controller.looking_down() {
			self.facing_pitch -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self.facing_pitch.clamp(-PI, PI);
		}
		if self.controller.looking_left() {
			self.facing_yaw += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		} else if self.controller.looking_right() {
			self.facing_yaw -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		}
	}
}