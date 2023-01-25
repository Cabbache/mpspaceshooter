use std::collections::HashMap;
use crate::Clients;
use serde::{Serialize,Deserialize,Serializer};
use serde::ser::SerializeStruct;
use serde_json::to_string;
use serde_json::from_str;
use warp::ws::Message;
use std::time::Instant;
//use std::hash::Hasher;
//use std::hash::Hash;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Color{
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub x: i32,
	pub y: i32,
	pub color: Color,
	pub keys: HashMap<GameKey, Option<Instant>>,
}

impl Serialize for PlayerState {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("PlayerState", 6)?;
		state.serialize_field("name", &self.name)?;
		state.serialize_field("public_id", &self.public_id)?;
		state.serialize_field("x", &self.x)?;
		state.serialize_field("y", &self.y)?;
		state.serialize_field("color", &self.color)?;
		state.serialize_field("keys", &self.keys.keys().filter(|&k| self.keys[k].is_some()).collect::<Vec<_>>())?;
		state.end()
	}
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

#[derive(Serialize, Debug)]
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
				let action_required = match clients.read().await.get(private_id){
					Some(v) => {
						let keystate = v.state.keys.get(&keyname);
						match keystate.and_then(|k| k.as_ref()).is_none(){
							true => !up,
							false => up
						}
					},
					_ => {
						eprintln!("Can't find client in hashmap");
						false
					}
				};
				if action_required{
					eprintln!("doing action");
					match clients.write().await.get_mut(private_id){
						Some(v) => v.state.keys.insert(keyname,
							match up{
								true => None,
								false => Some(Instant::now())
							}
						),
						_ => {
							eprintln!("Can't get write lock on clients");
							None
						}
					};

					let readlock = clients.read().await;
					let client = readlock.get(private_id).cloned().unwrap(); 
					eprintln!("{:?}",client.state);
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
}
