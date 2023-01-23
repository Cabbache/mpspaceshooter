use std::collections::HashSet;
use crate::Clients;
use serde::{Serialize,Deserialize};
use serde_json::to_string;
use serde_json::from_str;
use warp::ws::Message;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Color{
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub x: i32,
	pub y: i32,
	pub color: Color,
	pub keys: HashSet<GameKey>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum GameKey {
	ArrowUp,
	ArrowDown,
	ArrowLeft,
	ArrowRight,
	Space,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage{
	KeyUpdate {keyname: GameKey, up: bool},
	StateQuery,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage{
	GameState(Vec<PlayerState>),
	BroadCast {message: ClientMessage, from: String}
}

pub async fn handle_game_message(private_id: &str, message: &str, clients: &Clients){
	match from_str(message){
		Ok(v) => match v{
			ClientMessage::KeyUpdate { keyname, up } => {
				eprintln!("got keyupdate from {}: {:?}, {}", private_id, keyname, up);
				let has_effect = match clients.write().await.get_mut(private_id){
					Some(v) => {
						match up {
							true => v.state.keys.remove(&keyname),
							false => v.state.keys.insert(keyname)
						}
					},
					_ => {
						eprintln!("Can't get lock on clients");
						false
					}
				};
				if has_effect {
					let readlock = clients.read().await;
					let client = readlock.get(private_id).cloned().unwrap(); 
					let public_id = client.state.public_id;
					let broadcast = to_string(&ServerMessage::BroadCast {message: v, from: public_id}).unwrap();
					for (_, player) in readlock.iter(){
						player.sender.as_ref().unwrap().send(Ok(Message::text(broadcast.clone()))).unwrap();
					}
				}
			},
			ClientMessage::StateQuery => {
				eprintln!("Got statequery");
				let readlock = clients.read().await;
				let res = to_string(&ServerMessage::GameState(readlock.iter().map(|(_, value)| {
					value.state.clone()
				}).collect::<Vec<PlayerState>>())).unwrap();
				println!("{:?}", res);
				let client = readlock.get(private_id).cloned();
				client.unwrap().sender.unwrap().send(Ok(Message::text(res))).unwrap();
			},
		}
		_ => eprintln!("got something weird: {}", message),
	};
	//let topics_req: TopicsRequest = match from_str(&message) {
	//	Ok(v) => v,
	//	Err(e) => {
	//		eprintln!("error while parsing message to topics request: {}", e);
	//		return;
	//	}
	//};

	//if message == "ping" || message == "ping\n" {
	//	println!("got ping");
	//	let client = clients.read().await.get(id).cloned();
	//	client.unwrap().sender.unwrap().send(Ok(Message::text("pong")));
	//	return;
	//}

	//let mut locked = clients.write().await;
	//if let Some(v) = locked.get_mut(id) {
	//	v.topics = topics_req.topics;
	//}
}
