pub mod player;
mod world;

use std::{
	io::Write,
	net::{SocketAddr, TcpStream},
	time::Duration,
};

use cubegame_lib::{communication::message::*, WorldGenesisData};
use http::Uri;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

use player::Player;
use world::LoadedWorld;

pub struct Game {
	pub player: Player,
	pub world: LoadedWorld,
	/// Web socket connection to a server
	socket: WebSocket<MaybeTlsStream<TcpStream>>,
}
impl Game {
	pub fn new(server_url: Uri) -> Result<Game, ()> {
		let (mut socket, _) = match connect(&server_url) {
			Ok(r) => r,
			Err(e) => {
				log::error!("Failed to connect to game server at {}: {}", server_url, e);
				return Err(());
			}
		};

		let mut game = Game {
			player: Player::new(),
			world: LoadedWorld::new(),
			socket,
		};

		//game.send_msg(ServerMessage::CreateWorld(WorldGenesisData { seed: 123 }, "Test world".to_string()));

		Ok(game)
	}

	pub fn update(&mut self, dt: f32) {
		self.player.update(dt);
	}

	/// Cleaning up stuff
	pub fn shutdown(&mut self) {
		self.socket.close(None).unwrap();
	}

	/// Helper function to send a server message over the socket
	fn send_msg(&mut self, msg: ServerMessage) {
		// serialize message
		let data = rmp_serde::to_vec(&msg).unwrap();
		// send over socket
		self.socket.send(Message::Binary(data.into())).unwrap()
	}
}
