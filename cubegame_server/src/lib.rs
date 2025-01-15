use std::{
	net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener},
	thread,
};

use cubegame_lib::{
	communication::{Communication, ServerMessage, ServerResponse},
	BlockData, ChunkDeltaData, LocalBlockPos,
};
use tungstenite::{accept, Message};

pub struct ServerState {}

pub fn run_server(port: u16) -> Result<(), ()> {
	log::info!("Launching server on port {}", port);

	let listener = match TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port)) {
		Ok(l) => l,
		Err(e) => {
			log::error!("Failed to start server: {}", e);
			return Err(());
		}
	};

	for stream in listener.incoming() {
		thread::spawn(move || {
			let stream = match stream {
				Ok(s) => s,
				Err(_) => {
					return;
				}
			};
			let mut websocket = match accept(&stream) {
				Ok(s) => s,
				Err(e) => {
					log::error!("Error while accepting connection: {}", e);
					return;
				}
			};
			log::info!("New connection from {:?}", stream.peer_addr().unwrap());

			loop {
				let received = websocket.read().unwrap();

				match received {
					Message::Binary(data) => {
						let msg: ServerMessage = Communication::decode(&data);
						//log::debug!("{:?}", msg);
						websocket
							.send(Message::binary(make_response(&msg).encode()))
							.unwrap();
					}
					Message::Close(_) => {
						log::info!("Connection closed");
						break;
					}
					_ => {
						log::warn!("Unexpected message type received: {:?}", received);
					}
				}
			}
		});
	}
	Ok(())
}

/// Function to handle messages
fn make_response(msg: &ServerMessage) -> ServerResponse {
	match msg {
		ServerMessage::LoadChunk(chunk_pos) => {
			let mut delta = ChunkDeltaData::empty(*chunk_pos);
			delta
				.blocks
				.push((LocalBlockPos::new(2, 5, 2), BlockData::default()));
			ServerResponse::LoadChunkOK(delta)
		}
		_ => {
			log::warn!("Unhandled message, acknowledging: {:?}", msg);
			ServerResponse::Ack
		}
	}
}
