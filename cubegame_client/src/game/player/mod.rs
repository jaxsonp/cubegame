pub mod controller;

use std::f32::consts::PI;

use nalgebra::{Point3, Rotation3, Vector3};

use controller::PlayerController;
use cubegame_lib::{ChunkPos, CHUNK_WIDTH};

pub struct Player {
	/// position
	pub pos: Point3<f32>,
	/// View yaw from negative Z (0 -> 2PI)
	pub facing_yaw: f32,
	/// View pitch from horizon (-PI -> PI)
	pub facing_pitch: f32,
	/// Stores state on player input
	controller: PlayerController,
}
impl Player {
	/// Movement speed forward, in units per second
	const MOVE_SPEED_FORWARD: f32 = 9.0;
	/// Movement speed backward, in units per second
	const MOVE_SPEED_BACKWARD: f32 = 7.5;
	/// Movement speed laterally (strafing), in units per second
	const MOVE_SPEED_LATERAL: f32 = 8.0;
	/// Movement speed vertically, in units per second
	const MOVE_SPEED_VERTICAL: f32 = 8.0;

	/// look rotation speed, in degrees per second
	const LOOK_SPEED: f32 = 90.0;
	/// Angle to clamp the player's facing pitch, in degrees
	const PITCH_LIMIT: f32 = (PI / 2.0) - 0.01;

	pub fn new() -> Self {
		Self {
			controller: PlayerController::new(),
			pos: Point3::new(CHUNK_WIDTH as f32 / 2.0, 40.0, CHUNK_WIDTH as f32 / 2.0),
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
		yaw_rot * (pitch_rot * -Vector3::z()).normalize()
	}

	pub fn update(&mut self, dt: f32) {
		let up = Vector3::<f32>::y();
		let facing = self.facing_vec();

		// closures to remove unnecessary calcs
		let left = || up.cross(&facing).normalize();
		let forward = || left().cross(&up).normalize();

		if self.controller.forward() {
			self.pos += forward() * Self::MOVE_SPEED_FORWARD * dt;
		} else if self.controller.backward() {
			self.pos += -forward() * Self::MOVE_SPEED_BACKWARD * dt;
		}
		if self.controller.left() {
			self.pos += left() * Self::MOVE_SPEED_LATERAL * dt;
		} else if self.controller.right() {
			self.pos += -left() * Self::MOVE_SPEED_LATERAL * dt;
		}
		if self.controller.up() {
			self.pos += up * Self::MOVE_SPEED_VERTICAL * dt;
		} else if self.controller.down() {
			self.pos += -up * Self::MOVE_SPEED_VERTICAL * dt;
		}
		if self.controller.looking_up() {
			self.facing_pitch += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self
				.facing_pitch
				.clamp(-Player::PITCH_LIMIT, Player::PITCH_LIMIT);
		} else if self.controller.looking_down() {
			self.facing_pitch -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self
				.facing_pitch
				.clamp(-Player::PITCH_LIMIT, Player::PITCH_LIMIT);
		}
		if self.controller.looking_left() {
			self.facing_yaw += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		} else if self.controller.looking_right() {
			self.facing_yaw -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		}
	}

	/// Gets the chunk that this player is in
	pub fn chunk_pos(&self) -> ChunkPos {
		ChunkPos {
			x: (self.pos.x / (CHUNK_WIDTH as f32)).floor() as i32,
			z: (self.pos.z / (CHUNK_WIDTH as f32)).floor() as i32,
		}
	}
}
