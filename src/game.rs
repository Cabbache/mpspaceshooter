use crate::Clients;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::to_string;
use serde_json::from_str;
use warp::ws::Message;
use num_traits::checked_pow;
use std::time::Instant;
use std::f32::consts::FRAC_1_SQRT_2;
use xxhash_rust::xxh3::xxh3_64;

const UNITS_PER_SECOND: f32 = 30.0;

#[derive(Serialize, Debug, Clone)]
pub struct Color{
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub x: f32,
	pub y: f32,
	pub color: Color,
	pub motion: MotionStart,
}

impl Serialize for PlayerState {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
	{
		let mut state = serializer.serialize_struct("PlayerState", 6)?;
		state.serialize_field("name", &self.name)?;
		state.serialize_field("public_id", &self.public_id)?;
		state.serialize_field("color", &self.color)?;
		state.serialize_field("motion", &self.motion)?;

		let (x,y) = live_pos(self);
		state.serialize_field("x", &x)?;
		state.serialize_field("y", &y)?;
		state.end()
	}
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
	PlayerJoin(PlayerState),
	PlayerLeave(String),
	BroadCast {message: ClientMessage, from: String},
}

pub async fn broadcast(msg: &ServerMessage, clients: &Clients){
	let serialized_msg = to_string(msg).unwrap();
	for (_, player) in clients.read().await.iter(){
		let ch = player.sender.as_ref();
		match ch{
			Some(sender) => match sender.send(Ok(Message::text(serialized_msg.clone()))){
				Err(e) => eprintln!("Sending failed: {}", e),
				_ => {}
			},
			None => {}
		}
	}
}

fn live_pos(pstate: &PlayerState) -> (f32, f32){
	let mult = checked_pow(10, 9).unwrap();
	let now = Instant::now();
	let diff = ((now - pstate.motion.time).as_nanos() as f32) / (mult as f32);
	let (dx,dy) = match pstate.motion.direction{
		PlayerMotion::MoveUp => (0.0,diff),
		PlayerMotion::MoveDown => (0.0,-diff),
		PlayerMotion::MoveRight => (diff,0.0),
		PlayerMotion::MoveLeft => (-diff,0.0),
		PlayerMotion::MoveUpRight => (diff * FRAC_1_SQRT_2, diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveUpLeft => (-diff * FRAC_1_SQRT_2, diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveDownLeft => (-diff * FRAC_1_SQRT_2, -diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveDownRight => (diff * FRAC_1_SQRT_2, -diff * FRAC_1_SQRT_2),
		PlayerMotion::Stopped => (0.0,0.0)
	};
	let (dx,dy) = (dx*UNITS_PER_SECOND, dy*UNITS_PER_SECOND);
	(pstate.x + dx, pstate.y + dy)
}

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
						Some(v) => {
							let (nx,ny) = live_pos(&v.state);
							v.state.x = nx;
							v.state.y = ny;
							v.state.motion = MotionStart{
								direction: motion,
								time: Instant::now()
							};
						},
						_ => {
							eprintln!("Can't get write lock on clients");
						}
					};

					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					let msg = ServerMessage::BroadCast {message: v, from: public_id};
					broadcast(&msg, clients).await;
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