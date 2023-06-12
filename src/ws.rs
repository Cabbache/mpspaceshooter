use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use xxhash_rust::xxh3::xxh3_64;

use crate::game::handle_game_message;
use crate::game::ServerMessage;
use crate::WorldLoot;
//use crate::game::broadcast;
use crate::game::broadcast;

pub async fn client_connection(ws: WebSocket, private_id: String, clients: Clients, loot: WorldLoot, client: Client) {
	let (client_ws_sender, mut client_ws_rcv) = ws.split();
	let (client_sender, client_rcv) = mpsc::unbounded_channel();

	let client_rcv = UnboundedReceiverStream::new(client_rcv);
	tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
		if let Err(e) = result {
			eprintln!("error sending websocket msg: {}", e);
		}
	}));

	//client.sender = Some(client_sender);

	{
		let clr = clients.read().await;
		broadcast(
			&ServerMessage::PlayerJoin(client.state.read().await.clone()),
			&clr
		).await;
	}
	//clients.write().await.insert(private_id.clone(), client);
	clients.write().await.get_mut(&private_id.clone()).unwrap().sender = Some(client_sender);

	println!("{} connected", private_id);

	while let Some(result) = client_ws_rcv.next().await {
		let msg = match result {
			Ok(msg) => msg,
			Err(e) => {
				eprintln!("error receiving ws message for id: {}): {}", private_id.clone(), e);
				break;
			}
		};
		client_msg(&private_id, msg, &clients, &loot).await;
	}

	{
		let clr = clients.read().await;
		broadcast(
			&ServerMessage::PlayerLeave(format!("{:x}", xxh3_64(private_id.as_bytes()))),
			&clr
		).await;
	}
	clients.write().await.remove(&private_id);
	println!("{} disconnected", private_id);
}

async fn client_msg(private_id: &str, msg: Message, clients: &Clients, loot: &WorldLoot) {
	println!("received message from {}: {:?}", private_id, msg);
	let message = match msg.to_str() {
		Ok(v) => v,
		Err(_) => {
			eprintln!("msg.to_str failed");
			return;
		}
	};

	if let Err(e) = handle_game_message(private_id, message, clients, loot).await {
		eprintln!("Error handling game message: {}", e);
	}
	println!("exit handler {}", private_id);
}
