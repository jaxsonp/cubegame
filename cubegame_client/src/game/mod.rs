mod chunk;
pub mod player;

use std::{
	collections::HashMap,
	time::Instant,
	{net::TcpStream, time::Duration},
};

use cubegame_lib::{communication::*, ChunkPos};
use http::Uri;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};

use crate::render::mesh::mesher;
use chunk::LoadedChunk;
use player::Player;

/// Chunk render distance radius
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
		// connecting to server
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
						let _unloaded_chunk = self.chunks.remove(&chunk);
					}
				}
			}
		}
		Ok(())
	}

	pub fn check_remesh(&mut self) {
		for (_pos, chunk) in self.chunks.iter_mut() {
			if chunk.needs_remesh {
				/*if chunk.regenerate_meshes(renderer).is_err() {
					log::error!("Failed to remesh chunk at {pos}");
				}*/
				chunk.meshes = mesher::mesh_chunk(&chunk.data);
				chunk.needs_remesh = false;
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
			Ok(Communication::decode(&data))
		} else {
			log::error!("Received unexpected message: {:?}", received);
			Err(())
		}
	}
}
