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
use num_traits::Pow;

use crate::handler::spawn_from_prev;
use crate::Client;

const UNITS_PER_SECOND: f32 = 200.0; //player movement
const RADIANS_PER_SECOND: f32 = PI; //player rotation
const PLAYER_RADIUS: f32 = 25.0; //players have circular hitbox
const PISTOL_REACH: f32 = 500.0; //players have circular hitbox

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
	Grenade,
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
	Spawn,
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
	HealthUpdate {value: f32, from: String},
}

pub async fn broadcast(msg: &ServerMessage, clients_readlock: &tokio::sync::RwLockReadGuard<'_, HashMap<std::string::String, Client>>){
	let serialized_msg = to_string(msg).unwrap();
	println!("broadcast: fetching readlock on all clients");
	//for (_, player) in clients.read().await.iter(){
	for (_, player) in clients_readlock.iter(){
		let ch = player.sender.as_ref();
		match ch{
			Some(sender) => match sender.send(Ok(Message::text(serialized_msg.clone()))){
				Err(e) => eprintln!("Sending failed: {}", e),
				_ => {}
			},
			None => {}
		}
	}
	println!("broadcast: released readlock on all clients");
}

fn line_circle_intersect(xp: f32, yp: f32, xc:  f32, yc: f32, rot: f32) -> bool{
	//shift everything to make line start from origin
	let a = xc - xp;
	let b = yc - yp;
	let rot_90 = rot - PI/2f32;

	//compute the quadratic's 'b' coefficient (for variable r in polar form)
	let qb = -(2f32*a*rot_90.cos() + 2f32*b*rot_90.sin());
	let discriminant: f32 = qb.pow(2) - 4f32*(a.pow(2) + b.pow(2) - PLAYER_RADIUS.pow(2));
	if discriminant < 0f32{ //no real roots (no line-circle intersection)
		return false;
	}

	let root = discriminant.sqrt();

	let r1 = (root - qb)/2f32;
	let r2 = (-root - qb)/2f32;

	let r1_good = PISTOL_REACH > r1 && r1 > 0f32;
	let r2_good = PISTOL_REACH > r2 && r2 > 0f32;

	r1_good || r2_good
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
	let message: ClientMessage = match from_str(message) {
		Ok(v) => v,
		Err(m) => {
			eprintln!("Can't deserialize message: {}", m);
			return;
		}
	};

	println!("fetching read on all (enter handler) {}", private_id);
	let clr = clients.read().await;
	println!("fetched read on all (still acquired) {}", private_id);
	let sender_state = match clr.get(private_id) {
		Some(v) => &v.state,
		None => {
			eprintln!("Can't find sender in clients: {}", private_id);
			return;
		}
	};

	println!("fetching read on sender {}", private_id);
	let health = sender_state.read().await.clone().health;
	println!("fetched and released read on sender, {}", private_id);
	let is_allowed = match message {
		ClientMessage::StateQuery => true, //dead or alive, this is allowed
		ClientMessage::Spawn => health <= 0f32, //You have to be dead to call spawn
		_ => health > 0f32, //You have to be alive to call the rest
	};

	if !is_allowed {
		eprintln!("modded client detected (action doesn't match vital status)");
		return;
	}

	match message {
		ClientMessage::Spawn => {
			spawn_from_prev(&mut *sender_state.write().await);
		},
		ClientMessage::MotionUpdate { motion } => {
			println!("reading sender {}", private_id);
			if sender_state.read().await.clone().motion.direction == motion { //nothing changed
				eprintln!("modded client detected (redundant motion update)");
				return;
			}
			println!("read sender ok {}", private_id);
			let (nx, ny) = live_pos(&sender_state.read().await.clone());
			println!("read sender ok (2) {}", private_id);

			println!("writing sender {}", private_id);
			{
				let mut writeable = sender_state.write().await;
				writeable.x = nx;
				writeable.y = ny;
				writeable.motion = MotionStart{
					direction: motion,
					time: Instant::now()
				};
			} //This makes writable out of scope, so the write lock is released
			println!("write sender ok {}", private_id);

			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			let msg = ServerMessage::MotionUpdate {
				direction: motion,
				from: public_id,
				x: nx,
				y: ny,
			};
			//broadcast(&msg, clients).await;
			broadcast(&msg, &clr).await;
		},
		ClientMessage::RotationUpdate { direction } => { //TODO remove action_required variable since now we have locks within the hashmap, instead handle things inside the first match statement
			if sender_state.read().await.clone().rotation_motion.direction == direction {
				return;
			}
			let nr = live_rot(&sender_state.read().await.clone());

			let mut writeable = sender_state.write().await;
			writeable.rotation = nr;
			writeable.rotation_motion = RotationStart{
				direction: direction,
				time: Instant::now()
			};

			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			let msg = ServerMessage::RotationUpdate {
				direction: direction,
				from: public_id,
				r: nr
			};
			//broadcast(&msg, clients).await;
			broadcast(&msg, &clr).await;
		},
		ClientMessage::StateQuery => { //TODO rate limit this
			eprintln!("Got statequery");
			let mut players: Vec<PlayerState> = vec![];
			for (_, value) in clr.iter(){
				let cl = value.state.read().await.clone();
				players.push(cl);
			}

			let res = to_string(
				&ServerMessage::GameState(players)
			).unwrap();

			//very ugly
			match clr.get(private_id) {
				Some(v) => v.sender.as_ref().unwrap().send(Ok(Message::text(res))).unwrap(),
				None => eprintln!("Can't find client")
			}
		},
		ClientMessage::ChangeSlot { slot } => { //TODO consider also changing trigger_pressed to false both on client and server when changeslot
			if sender_state.read().await.clone().inventory.selection != slot
				{ sender_state.write().await.inventory.selection = slot; }
		},
		ClientMessage::TrigUpdate { pressed } => {
			println!("reading client state {}", private_id);
			let cl = sender_state.read().await.clone();
			println!("read ok {}", private_id);
			if cl.trigger_pressed == pressed {
				return;
			}
			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			println!("writing client state {}", private_id);
			sender_state.write().await.trigger_pressed = pressed;
			println!("write ok {}", private_id);
			let weapon_type = cl.inventory.weapons.get( //TODO since we are not using .read() it might be outdated
				&cl.inventory.selection
			).unwrap().weptype.clone();

			//TODO since we don't care if pistol released, we should make the client not even send it
			if !pressed{ //this is temporary?
				return;
			}

			//TODO check ammo > 0

			println!("broadcasting {}", private_id);
			broadcast(
				&ServerMessage::TrigUpdate {
					by: public_id,
					weptype: weapon_type.clone(),
					pressed: pressed,
				},
				//clients
				&clr
			).await;

			//TODO decrement ammo

			//Update healths
			match weapon_type {
				WeaponType::Pistol => {
					let rr = live_rot(&cl);
					let (sx, sy) = live_pos(&cl);

					//TODO important idea: have the client tell server which opponent the bullet hits, server only checks if its true.
					//flaw: client can lie and say it hit someone who is behind actual hit
					//boring linear search
					for (key, value) in clr.iter() { //TODO try using for_each
						if key == private_id{ //Can't shoot yourself
							continue;
						}

						println!("copying player state for testing collision");
						let playerstate = value.state.read().await.clone();
						println!("collision copy ok");

						//ignore dead players
						if playerstate.health <= 0f32 {
							continue;
						}
						let (px, py) = live_pos(&playerstate);
						let hit = line_circle_intersect(sx, sy, px, py, rr);
						if !hit{
							continue;
						}

						println!("writing health {} (for {})", private_id, key);
						let new_health = {
							let mut writeable = value.state.write().await;
							writeable.health -= 10f32; //hard coded pistol damage to 10
							//playerstate.health -= 10f32; //update the copy too
							writeable.health
						};
						println!("written health {}", private_id);

						println!("broadcasting health {}", private_id);
						broadcast(
							&ServerMessage::HealthUpdate {
								value: new_health,
								from: format!("{:x}", xxh3_64(key.as_bytes()))
							},
							//clients
							&clr
						).await;
						println!("broadcasted health{}", private_id);
						
						break; //important! you can only hit one player at one time
					};
				},
				_=>{}
			}
			println!("updated healths, {}", private_id);
		}
	}
}
