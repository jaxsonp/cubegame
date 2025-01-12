mod chunk;
pub mod player;

use cubegame_lib::ChunkPos;
use http::Uri;
use std::{
	collections::HashMap,
	time::Instant,
	{net::TcpStream, time::Duration},
};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

use crate::render::Renderer;
use chunk::LoadedChunk;
use cubegame_lib::communication::*;
use player::Player;

/// Radius of chunk render distance
const RENDER_DISTANCE: u32 = 4;

pub struct Game {
	pub player: Player,
	/// Loaded chunks
	pub chunks: HashMap<ChunkPos, LoadedChunk>,
	/// Web socket connection to a game server
	socket: WebSocket<MaybeTlsStream<TcpStream>>,
	/// For ticking once per second
	last_slow_tick: Instant,
}
impl Game {
	pub fn new(server_url: Uri) -> Result<Game, ()> {
		let (socket, _addr) = match connect(&server_url) {
			Ok(r) => r,
			Err(e) => {
				log::error!("Failed to connect to game server at {}: {}", server_url, e);
				return Err(());
			}
		};

		Ok(Game {
			player: Player::new(),
			chunks: HashMap::new(),
			socket,
			last_slow_tick: Instant::now(),
		})

		//game.send_msg(ServerMessage::CreateWorld(WorldGenesisData { seed: 123 }, "Test world".to_string()));
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);

		if self.last_slow_tick.elapsed() > Duration::from_secs(1) {
			println!("player: {} {}", self.player.pos, self.player.chunk_pos());
			self.last_slow_tick = Instant::now();
			if self.load_chunks().is_err() {
				log::error!("Error while loading/unloading chunks");
			}
		}
	}

	/// Cleaning up stuff
	pub fn shutdown(&mut self) {
		self.socket.close(None).unwrap();
	}

	/// Loads/unloads chunks based on player position
	fn load_chunks(&mut self) -> Result<(), ()> {
		// chunk that player is in
		let player_chunk = self.player.chunk_pos();

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
					if !self.chunks.contains_key(&chunk) {
						log::info!("Loading chunk {}", chunk);
						self.send_msg(ServerMessage::LoadChunk(chunk));

						let response = self.recv_response()?;
						if let ServerResponse::LoadChunkOK(data) = response {
							self.chunks
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
					if self.chunks.contains_key(&chunk) {
						log::debug!("Unloading chunk {}", chunk);
						let _unloaded_chunk = self.chunks.remove(&chunk);
					}
				}
			}
		}
		Ok(())
	}

	pub fn check_remesh(&mut self, renderer: &Renderer) {
		for (pos, chunk) in self.chunks.iter_mut() {
			if chunk.needs_remesh {
				if chunk.regenerate_meshes(renderer).is_err() {
					log::error!("Failed to remesh chunk at {pos}");
				}
			}
		}
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
			Ok(rmp_serde::from_slice(&data).unwrap())
		} else {
			log::error!("Received unexpected message: {:?}", received);
			Err(())
		}
	}
}
