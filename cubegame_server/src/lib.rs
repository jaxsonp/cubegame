use std::{io::Read, thread, net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener}};

use tungstenite::{accept, Message};
use cubegame_lib::communication::message::ServerMessage;

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
			let stream = stream.unwrap();
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
						let msg: ServerMessage = rmp_serde::decode::from_slice(&data).unwrap();
						log::info!("{:?}", msg);
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