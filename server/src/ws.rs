use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

use crate::game::broadcast;
use crate::game::handle_game_message;
use crate::WorldLoot;
use utils::server_gameobjects::ServerMessage;

pub async fn client_connection(
	ws: WebSocket,
	public_id: String,
	clients: Clients,
	loot: WorldLoot,
	client: Client,
) {
	let (client_ws_sender, mut client_ws_rcv) = ws.split();
	let (client_sender, client_rcv) = mpsc::unbounded_channel();

	let client_rcv = UnboundedReceiverStream::new(client_rcv);
	tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
		if let Err(e) = result {
			eprintln!("error sending websocket msg: {}", e);
		}
	}));

	{
		let clr = clients.read().await;
		broadcast(
			&ServerMessage::PlayerJoin(client.state.read().await.clone()),
			&clr,
		)
		.await;
	}
	clients
		.write()
		.await
		.get_mut(&public_id.clone())
		.unwrap()
		.sender = Some(client_sender);

	println!("{} connected", public_id);

	while let Some(result) = client_ws_rcv.next().await {
		let msg = match result {
			Ok(msg) => msg,
			Err(e) => {
				eprintln!(
					"error receiving ws message for id: {}): {}",
					public_id.clone(),
					e
				);
				break;
			}
		};

		//TODO make client_msg with a rate limiter or cheat detection, exit this loop if triggered
		client_msg(&public_id, msg, &clients, &loot).await;
	}

	{
		let clr = clients.read().await;
		broadcast(&ServerMessage::PlayerLeave(public_id.clone()), &clr).await;
	}
	clients.write().await.remove(&public_id);
	println!("{} disconnected", public_id);
}

async fn client_msg(public_id: &String, msg: Message, clients: &Clients, loot: &WorldLoot) {
	println!("received message from {}: {:?}", public_id, msg);
	let message = match msg.to_str() {
		Ok(v) => v,
		Err(_) => {
			eprintln!("msg.to_str failed");
			return;
		}
	};

	if let Err(e) = handle_game_message(public_id.clone(), message, clients, loot).await {
		eprintln!("Error handling game message: {}", e);
	}
	println!("exit handler {}", public_id);
}
