use crate::{Client, Clients};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use xxhash_rust::xxh3::xxh3_64;

use crate::game::handle_game_message;
use crate::game::ServerMessage;
//use crate::game::broadcast;
use crate::game::broadcast;

pub async fn client_connection(ws: WebSocket, private_id: String, clients: Clients, client: Client) {
	let (client_ws_sender, mut client_ws_rcv) = ws.split();
	let (client_sender, client_rcv) = mpsc::unbounded_channel();

	let client_rcv = UnboundedReceiverStream::new(client_rcv);
	tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
		if let Err(e) = result {
			eprintln!("error sending websocket msg: {}", e);
		}
	}));

	//client.sender = Some(client_sender);

	println!("broadcasting original (join)");
	{
		let clr = clients.read().await;
		broadcast(
			&ServerMessage::PlayerJoin(client.state.read().await.clone()),
			&clr
		).await;
	}
	println!("broadcasted original");

	//clients.write().await.insert(private_id.clone(), client);
	println!("obtaining write lock for sender {}", private_id);
	clients.write().await.get_mut(&private_id.clone()).unwrap().sender = Some(client_sender);
	println!("obtained and released write lock for sender, {}", private_id);

	println!("{} connected", private_id);

	while let Some(result) = client_ws_rcv.next().await {
		let msg = match result {
			Ok(msg) => msg,
			Err(e) => {
				eprintln!("error receiving ws message for id: {}): {}", private_id.clone(), e);
				break;
			}
		};
		client_msg(&private_id, msg, &clients).await;
	}

	println!("broadcasting original (leave)");
	{
		let clr = clients.read().await;
		broadcast(
			&ServerMessage::PlayerLeave(format!("{:x}", xxh3_64(private_id.as_bytes()))),
			&clr
		).await;
	}
	println!("broadcasted original (leave)");
	clients.write().await.remove(&private_id);
	println!("{} disconnected", private_id);
}

async fn client_msg(private_id: &str, msg: Message, clients: &Clients) {
	println!("received message from {}: {:?}", private_id, msg);
	let message = match msg.to_str() {
		Ok(v) => v,
		Err(_) => {
			eprintln!("msg.to_str failed");
			return;
		}
	};

	handle_game_message(private_id, message, clients).await;
	println!("exit handler {}", private_id);
}
