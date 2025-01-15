use std::f32::consts::PI;

use nalgebra::{Matrix4, Point3, Rotation3, Vector3};

use crate::game::controller::PlayerController;
use cubegame_lib::{ChunkPos, CHUNK_WIDTH};

#[derive(Debug, Copy, Clone)]
pub struct Player {
	/// position
	pub pos: [f32; 3],
	/// View yaw from negative Z (0 -> 2PI)
	pub facing_yaw: f32,
	/// View pitch from horizon (-PI -> PI)
	pub facing_pitch: f32,
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
			pos: [CHUNK_WIDTH as f32 / 2.0, 40.0, CHUNK_WIDTH as f32 / 2.0],
			facing_yaw: 0.0,
			facing_pitch: 0.0,
		}
	}

	/// Unit vector representing direction player is facing
	pub fn facing_vec(&self) -> Vector3<f32> {
		let pitch_rot = Rotation3::from_axis_angle(&Vector3::x_axis(), self.facing_pitch);
		let yaw_rot = Rotation3::from_axis_angle(&Vector3::y_axis(), self.facing_yaw);
		yaw_rot * (pitch_rot * -Vector3::z()).normalize()
	}

	pub fn update(&mut self, dt: f32, controller: &PlayerController) {
		let up = Vector3::<f32>::y();
		let facing = self.facing_vec();

		// closures to remove unnecessary calcs
		let left = || up.cross(&facing).normalize();
		let forward = || left().cross(&up).normalize();

		let mut movement = Vector3::<f32>::zeros();
		let mut moved = false;
		if controller.forward() {
			movement += forward() * Self::MOVE_SPEED_FORWARD * dt;
			moved = true;
		} else if controller.backward() {
			movement += -forward() * Self::MOVE_SPEED_BACKWARD * dt;
			moved = true;
		}
		if controller.left() {
			movement += left() * Self::MOVE_SPEED_LATERAL * dt;
			moved = true;
		} else if controller.right() {
			movement += -left() * Self::MOVE_SPEED_LATERAL * dt;
			moved = true;
		}
		if controller.up() {
			movement += up * Self::MOVE_SPEED_VERTICAL * dt;
			moved = true;
		} else if controller.down() {
			movement += -up * Self::MOVE_SPEED_VERTICAL * dt;
			moved = true;
		}
		if moved {
			let new_pos = Vector3::from(self.pos) + movement;
			self.pos = new_pos.as_slice().try_into().unwrap();
		}

		if controller.looking_up() {
			self.facing_pitch += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self
				.facing_pitch
				.clamp(-Player::PITCH_LIMIT, Player::PITCH_LIMIT);
		} else if controller.looking_down() {
			self.facing_pitch -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_pitch = self
				.facing_pitch
				.clamp(-Player::PITCH_LIMIT, Player::PITCH_LIMIT);
		}
		if controller.looking_left() {
			self.facing_yaw += Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		} else if controller.looking_right() {
			self.facing_yaw -= Self::LOOK_SPEED.to_radians() * dt;
			self.facing_yaw %= PI * 2.0;
		}
	}

	/// Gets the chunk that this player is in
	pub fn chunk_pos(&self) -> ChunkPos {
		ChunkPos {
			x: (self.pos[0] / (CHUNK_WIDTH as f32)).floor() as i32,
			z: (self.pos[2] / (CHUNK_WIDTH as f32)).floor() as i32,
		}
	}

	pub fn view_matrix(&self) -> Matrix4<f32> {
		let pos: Vector3<f32> = self.pos.into();
		let target: Point3<f32> = (pos + self.facing_vec()).into();
		Matrix4::look_at_rh(&pos.into(), &target, &Vector3::y_axis())
	}
}
