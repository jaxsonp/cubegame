mod chunk;
pub mod controller;
pub mod player;
pub mod world;

use std::sync::Arc;
use std::{
	time::Instant,
	{net::TcpStream, time::Duration},
};

use cubegame_lib::{communication::*, ChunkPos};
use http::Uri;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use winit::event::{DeviceEvent, ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window};

use crate::render::{mesher, Renderer};
use chunk::LoadedChunk;
use controller::PlayerController;
use world::WorldData;

/// Chunk render distance radius
const RENDER_DISTANCE: u32 = 8;

/// Struct that represents everything to run the actual cubegame
pub struct Game {
	window: Arc<Window>,
	pub world_data: WorldData,
	controller: PlayerController,
	/// Web socket connection to a game server
	socket: WebSocket<MaybeTlsStream<TcpStream>>,
	/// For ticking once per second
	last_slow_tick: Instant,
	in_menu: bool,
}
impl Game {
	pub fn new(server_url: Uri, window: Arc<Window>) -> Result<Game, ()> {
		// connecting to server
		let (socket, _addr) = match connect(&server_url) {
			Ok(r) => r,
			Err(e) => {
				log::error!("Failed to connect to game server at {}: {}", server_url, e);
				return Err(());
			}
		};
		log::info!("Connected to game server at {}", server_url);

		let game = Game {
			window,
			world_data: WorldData::new(),
			controller: PlayerController::new(),
			socket,
			last_slow_tick: Instant::now(),
			in_menu: false,
		};
		if game.window.has_focus() {
			game.grab_cursor();
		}
		return Ok(game);
	}

	pub fn update(&mut self, dt: f32) {
		// updating player from inputs
		self.world_data.player.update(dt, &mut self.controller);

		if self.last_slow_tick.elapsed() > Duration::from_secs(1) {
			self.last_slow_tick = Instant::now();
			let res = self.load_chunks();
			if res.is_err() {
				log::error!("Error while loading/unloading chunks");
			}
		}
	}

	/// Cleaning up stuff
	pub fn shutdown(&mut self) {
		self.socket.close(None).unwrap();
		self.release_cursor();
	}

	/// Loads/unloads chunks based on player position
	fn load_chunks(&mut self) -> Result<(), ()> {
		// chunk that player is in
		let player_chunk = self.world_data.player.chunk_pos();

		let render_dist = RENDER_DISTANCE as i32;
		for x in (-render_dist)..=render_dist {
			for z in (-render_dist)..=render_dist {
				// relative chunk position
				let chunk = ChunkPos {
					x: x + player_chunk.x,
					z: z + player_chunk.z,
				};

				let dist = ((x.pow(2) + z.pow(2)) as f32).sqrt();
				if dist < RENDER_DISTANCE as f32 {
					// chunk should be loaded
					if !self.world_data.chunks.contains_key(&chunk) {
						self.send_msg(ServerMessage::LoadChunk(chunk));
						let response = self.recv_response()?;

						if let ServerResponse::LoadChunkOK(data) = response {
							self.world_data
								.chunks
								.insert(chunk, LoadedChunk::load_from_delta(data));
						} else {
							log::error!(
								"Received unexpected response while requesting chunk data: {:?}",
								response
							);
							return Err(());
						}
					}
				} else {
					// chunk does not need to be loaded
					if self.world_data.chunks.contains_key(&chunk) {
						let _unloaded_chunk = self.world_data.chunks.remove(&chunk);
					}
				}
			}
		}
		Ok(())
	}

	pub fn handle_device_event(&mut self, event: &DeviceEvent) {
		if self.window.has_focus() {
			if !self.in_menu {
				self.controller.handle_input(event);
			}

			match event {
				_ => {}
			}
		}
	}

	pub fn handle_window_event(&mut self, event: &WindowEvent) {
		match event {
			WindowEvent::Focused(has_focus) => {
				if !*has_focus {
					self.release_cursor();
					self.in_menu = true;
				}
			}
			WindowEvent::KeyboardInput {
				device_id: _,
				event:
					KeyEvent {
						physical_key: PhysicalKey::Code(KeyCode::Escape),
						state: ElementState::Pressed,
						repeat: false,
						..
					},
				is_synthetic: false,
			} => {
				if self.in_menu {
					self.close_menu()
				} else {
					self.open_menu()
				}
			}
			_ => {}
		}
	}

	/// Remeshes chunks if they need to be, also binds meshes' local bind groups
	pub fn prep_meshes(&mut self, renderer: &Renderer) {
		for (_pos, chunk) in self.world_data.chunks.iter_mut() {
			// remeshing chunks
			if chunk.needs_remesh {
				chunk.meshes = mesher::generate_chunk_meshes(&chunk.data);
				chunk.needs_remesh = false;
			}
			for mesh in chunk.meshes.iter_mut() {
				mesh.load_buffers(renderer);
			}
			chunk.border_lines.load_buffers(renderer);
		}
	}

	fn open_menu(&mut self) {
		self.in_menu = true;
		self.release_cursor();
		self.controller.reset();
	}

	fn close_menu(&mut self) {
		self.in_menu = false;
		self.grab_cursor();
	}

	fn grab_cursor(&self) {
		self.window.set_cursor_visible(false);
		self.window
			.set_cursor_grab(CursorGrabMode::Confined)
			.or_else(|_e| self.window.set_cursor_grab(CursorGrabMode::Locked))
			.unwrap();
	}

	fn release_cursor(&self) {
		self.window.set_cursor_visible(true);
		self.window.set_cursor_grab(CursorGrabMode::None).unwrap()
	}

	/// Helper function to send server messages
	fn send_msg(&mut self, msg: ServerMessage) {
		// serialize message
		// send over socket
		self.socket
			.send(Message::Binary(msg.encode().into()))
			.unwrap()
	}

	/// Helper function to receive server responses
	fn recv_response(&mut self) -> Result<ServerResponse, ()> {
		let received = self.socket.read().unwrap();
		if let Message::Binary(data) = received {
			Ok(Communication::decode(&data))
		} else {
			log::error!("Received unexpected message: {:?}", received);
			Err(())
		}
	}
}
