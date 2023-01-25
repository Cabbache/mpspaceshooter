use crate::Clients;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::to_string;
use serde_json::from_str;
use warp::ws::Message;
use std::time::Instant;

const UNITS_PER_SECOND: f64 = 30.0;

#[derive(Serialize, Debug, Clone)]
pub struct Color{
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub x: i32,
	pub y: i32,
	pub color: Color,
	pub motion: MotionStart,
}

#[derive(Debug, Clone)]
pub struct MotionStart{
	pub direction: PlayerMotion,
	pub time: Instant,
}

impl Serialize for MotionStart {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
	{
		let mut state = serializer.serialize_struct("MotionStart", 1)?;
		state.serialize_field("direction", &self.direction)?;
		state.end()
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum PlayerMotion {
	MoveUp,
	MoveDown,
	MoveLeft,
	MoveRight,
	MoveUpRight,
	MoveDownRight,
	MoveDownLeft,
	MoveUpLeft,
	Stopped
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage{
	MotionUpdate {motion: PlayerMotion},
	StateQuery,
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage{
	GameState(Vec<PlayerState>),
	BroadCast {message: ClientMessage, from: String}
}

//fn live_pos(pstate: PlayerState) -> (i32, i32){
//	let now = Instant::now();
//}

pub async fn handle_game_message(private_id: &str, message: &str, clients: &Clients){
	match from_str(message){
		Ok(v) => match v{
			ClientMessage::MotionUpdate { motion } => {
				eprintln!("got keyupdate from {}: {:?}", private_id, motion);
				let action_required = match clients.read().await.get(private_id){
					Some(v) => v.state.motion.direction != motion,
					_ => {
						eprintln!("Can't find client in hashmap");
						false
					}
				};
				if action_required{
					match clients.write().await.get_mut(private_id){
						Some(v) => v.state.motion = MotionStart{
							direction: motion,
							time: Instant::now()
						},
						_ => {
							eprintln!("Can't get write lock on clients");
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
