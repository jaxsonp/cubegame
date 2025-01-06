use std::time::{Duration, Instant};

/// Struct for managing and calculating framerate
pub struct FramerateManager {
	min_frame_time: Duration,
	last_frame: Instant,
	/// Number of frames since the last fps check
	fps_counter: u32,
	/// Time of last fps check
	last_fps_check: Instant,
	/// Current FPS, updated every second
	pub current_fps: u32,
}
impl FramerateManager {
	pub fn new() -> FramerateManager {
		Self {
			min_frame_time: Duration::from_secs(0),
			last_frame: Instant::now(),
			fps_counter: 0,
			last_fps_check: Instant::now(),
			current_fps: 0,
		}
	}

	pub fn set_max_fps(&mut self, fps: u64) {
		let fps: f32 = fps as f32;
		self.min_frame_time = Duration::from_micros((1_000_000.0 / fps) as u64);
	}

	/// The magic
	pub fn tick(&mut self) {
		// sleeping to hit target fps
		let elapsed = self.last_frame.elapsed();
		if elapsed < self.min_frame_time {
			let remaining = self.min_frame_time - elapsed;
			spin_sleep::sleep(remaining);
		}
		self.last_frame = Instant::now();
	
		// counting fps
		if self.last_fps_check.elapsed() >= Duration::from_secs(1) {
			self.current_fps = self.fps_counter;
			self.fps_counter = 0;
			self.last_fps_check = Instant::now();
		}
		self.fps_counter += 1;
	}
}