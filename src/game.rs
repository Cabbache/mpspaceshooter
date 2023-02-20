use crate::Clients;
use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeStruct;
use serde_json::to_string;
use serde_json::from_str;
use warp::ws::Message;
use num_traits::checked_pow;
use std::collections::HashMap;
use std::time::Instant;
use std::f32::consts::FRAC_1_SQRT_2;
use std::f32::consts::PI;
use xxhash_rust::xxh3::xxh3_64;

const UNITS_PER_SECOND: f32 = 200.0;
const RADIANS_PER_SECOND: f32 = PI;
const PLAYER_RADIUS: f32 = 25.0;

#[derive(Serialize, Debug, Clone)]
pub struct Color{
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Debug, Clone)]
pub struct Weapon{
	pub weptype: WeaponType,
	pub ammo: u32,
}

#[derive(Debug, Clone, Serialize)]
pub enum WeaponType{
	Pistol,
	FlameThrower,
}

#[derive(Debug, Clone)]
pub struct Inventory{
	pub selection: u8,
	pub weapons: HashMap<u8, Weapon>,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub health: f32,
	pub x: f32,
	pub y: f32,
	pub rotation: f32,
	pub color: Color,
	pub inventory: Inventory,
	pub motion: MotionStart,
	pub rotation_motion: RotationStart,
	pub trigger_pressed: bool,
}

impl Serialize for PlayerState {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
	{
		let mut state = serializer.serialize_struct("PlayerState", 9)?;
		state.serialize_field("name", &self.name)?;
		state.serialize_field("public_id", &self.public_id)?;
		state.serialize_field("color", &self.color)?;
		state.serialize_field("motion", &self.motion)?;
		state.serialize_field("health", &self.health)?;
		state.serialize_field("rotation_motion", &self.rotation_motion)?;

		let (x,y) = live_pos(self);
		let r = live_rot(self);
		state.serialize_field("x", &x)?;
		state.serialize_field("y", &y)?;
		state.serialize_field("rotation", &r)?;
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

#[derive(Debug, Clone)]
pub struct RotationStart{
	pub direction: PlayerRotation,
	pub time: Instant,
}

impl Serialize for RotationStart {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where S: Serializer,
	{
		let mut state = serializer.serialize_struct("RotationStart", 1)?;
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

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum PlayerRotation {
	AntiClockwise,
	Clockwise,
	Stopped
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage{
	MotionUpdate {motion: PlayerMotion},
	RotationUpdate {direction: PlayerRotation},
	ChangeSlot {slot: u8},
	TrigUpdate {pressed: bool},
	StateQuery,
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage{
	GameState(Vec<PlayerState>),
	PlayerJoin(PlayerState),
	PlayerLeave(String),
	MotionUpdate {direction: PlayerMotion, from: String, x: f32, y: f32},
	RotationUpdate {direction: PlayerRotation, from: String, r: f32},
	TrigUpdate {by: String, weptype: WeaponType, pressed: bool},
	HealthUpdate {value: f32},
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
		PlayerMotion::MoveUp => (0.0,-diff),
		PlayerMotion::MoveDown => (0.0,diff),
		PlayerMotion::MoveRight => (diff,0.0),
		PlayerMotion::MoveLeft => (-diff,0.0),
		PlayerMotion::MoveUpRight => (diff * FRAC_1_SQRT_2, -diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveUpLeft => (-diff * FRAC_1_SQRT_2, -diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveDownLeft => (-diff * FRAC_1_SQRT_2, diff * FRAC_1_SQRT_2),
		PlayerMotion::MoveDownRight => (diff * FRAC_1_SQRT_2, diff * FRAC_1_SQRT_2),
		PlayerMotion::Stopped => (0.0,0.0)
	};
	let (dx,dy) = (dx*UNITS_PER_SECOND, dy*UNITS_PER_SECOND);
	(pstate.x + dx, pstate.y + dy)
}

fn live_rot(pstate: &PlayerState) -> f32 {
	let mult = checked_pow(10, 9).unwrap();
	let now = Instant::now();
	let diff = ((now - pstate.rotation_motion.time).as_nanos() as f32) / (mult as f32);
	let dr = match pstate.rotation_motion.direction{
		PlayerRotation::AntiClockwise => -diff,
		PlayerRotation::Clockwise => diff,
		PlayerRotation::Stopped => 0.0
	} * RADIANS_PER_SECOND;
	pstate.rotation + dr
}

pub async fn handle_game_message(private_id: &str, message: &str, clients: &Clients){
	match from_str(message){
		Ok(v) => match v{
			ClientMessage::MotionUpdate { motion } => {
				eprintln!("got keyupdate from {}: {:?}", private_id, motion);
				let (mut nx, mut ny) = (0.0,0.0);
				let action_required = match clients.read().await.get(private_id){
					Some(v) => {
						let cl = v.state.read().await.clone();
						(nx, ny) = live_pos(&cl);
						cl.motion.direction != motion
					},
					_ => {
						eprintln!("Can't find client in hashmap");
						false
					}
				};
				if action_required{
					match clients.read().await.get(private_id){
						Some(v) => {
							let mut writeable = v.state.write().await;
							writeable.x = nx;
							writeable.y = ny;
							writeable.motion = MotionStart{
								direction: motion,
								time: Instant::now()
							};
						},
						_ => {
							eprintln!("Can't get write lock on clients in motionupdate");
						}
					};

					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					let msg = ServerMessage::MotionUpdate {
						direction: motion,
						from: public_id,
						x: nx,
						y: ny,
					};
					broadcast(&msg, clients).await;
				}
			},
			ClientMessage::RotationUpdate { direction } => { //TODO remove action_required variable since now we have locks within the hashmap, instead handle things inside the first match statement
				eprintln!("got rotation update from {}: {:?}", private_id, direction);
				let mut nr = 0.0;
				let action_required = match clients.read().await.get(private_id){
					Some(v) => {
						let cl = v.state.read().await.clone();
						nr = live_rot(&cl);
						cl.rotation_motion.direction != direction
					},
					_ => {
						eprintln!("Can't find client in hashmap");
						false
					}
				};
				if action_required{
					match clients.read().await.get(private_id){
						Some(v) => {
							let mut writeable = v.state.write().await;
							writeable.rotation = nr;
							writeable.rotation_motion = RotationStart{
								direction: direction,
								time: Instant::now()
							};
						},
						_ => {
							eprintln!("Can't find client in clients (rotation update)");
						}
					};

					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					let msg = ServerMessage::RotationUpdate {
						direction: direction,
						from: public_id,
						r: nr
					};
					broadcast(&msg, clients).await;
				}
			},
			ClientMessage::StateQuery => {
				eprintln!("Got statequery");
				let mut players: Vec<PlayerState> = vec![];
				let readlock = clients.read().await;
				for (_, value) in readlock.iter(){
					let cl = value.state.read().await.clone();
					players.push(cl);
				}
				let res = to_string(
					&ServerMessage::GameState(players)
				).unwrap();
				let client = readlock.get(private_id).cloned();
				client.unwrap().sender.unwrap().send(Ok(Message::text(res))).unwrap();
			},
			ClientMessage::ChangeSlot { slot } => {
				match clients.read().await.get(private_id){
					Some(v) => {
						let cl = v.state.read().await.clone();
						if cl.inventory.selection != slot
							{ v.state.write().await.inventory.selection = slot; }
					},
					None => eprintln!("Can't find client in clients")
				}
			},
			ClientMessage::TrigUpdate { pressed } => {
				match clients.read().await.get(private_id){
					Some(v) => {
						let cl = v.state.read().await.clone();
						if cl.trigger_pressed == pressed {
							return;
						}
						let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
						v.state.write().await.trigger_pressed = pressed;
						let weapon_type = cl.inventory.weapons.get( //TODO since we are not using .read() it might be outdated
							&cl.inventory.selection
						).unwrap().weptype.clone();
						broadcast(
							&ServerMessage::TrigUpdate {
								by: public_id,
								weptype: weapon_type,
								pressed: pressed,
							},
							clients
						).await;
					},
					None => eprintln!("Can't find client in clients")
				}
			},
		}
		_ => eprintln!("got something weird: {}", message),
	};
}
