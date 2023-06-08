use std::collections::HashMap;
use std::f32::consts::{FRAC_1_SQRT_2, PI};
use std::time::Instant;

use futures::future::join_all;
use num_traits::{checked_pow, Pow};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use serde_json::{from_str, json, to_string, Value};
use warp::ws::Message;
use xxhash_rust::xxh3::xxh3_64;

use crate::Clients;
use crate::Client;
use crate::handler::spawn_from_prev;

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

#[derive(Serialize, Debug, Clone)]
pub struct Weapon{
	pub weptype: WeaponType,
	pub ammo: u32,
}

#[derive(Debug, Clone)]
pub enum WeaponType{
	Pistol,
	Grenade {press_time: Instant}
}

impl Serialize for WeaponType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			WeaponType::Pistol => serializer.serialize_str("Pistol"),
			WeaponType::Grenade{ press_time: _ } => serializer.serialize_str("Grenade"),
		}
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct Inventory{
	pub selection: u8,
	pub weapons: HashMap<u8, Weapon>,
}

#[derive(Serialize, Debug, Clone)]
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

impl PlayerState {
	pub fn encode_other(&self) -> Value{
		let (x,y) = live_pos(self);
		let r = live_rot(self);
		return json!({
			"name": &self.name,
			"public_id": &self.public_id,
			"color": &self.color,
			"motion": &self.motion,
			"rotation_motion": &self.rotation_motion,
			"x": x,
			"y": y,
			"rotation": r,
		});
	}

	pub fn encode(&self, as_self: bool) -> Value{
		if !as_self {
			return self.encode_other();
		}
		let mut result = self.encode_other();
		let additional = json!({
			"inventory": &self.inventory,
			"health": &self.health,
		});
		result.as_object_mut().unwrap().extend(additional.as_object().unwrap().clone());
		return result;
	}
}

impl Client {
	pub async fn transmit(&self, msg: &ServerMessage, public_id: Option<String>){
		let ch = self.sender.as_ref();
		if ch.is_none(){
			return;
		}
		let ch = ch.unwrap();
		let serialized_msg = match msg {
			ServerMessage::GameState(_) |
			ServerMessage::PlayerJoin(_) => {
				let public_id = match public_id {
					Some(id) => id,
					None => self.state.read().await.public_id.clone()
				};
				match msg {
					ServerMessage::GameState(pstates) => {
						let encoded_states: Vec<Value> = pstates.iter().map(|state| {
							state.encode(public_id == state.public_id)
						}).collect();
						to_string(
							&json!({
								"t": "GameState",
								"c": encoded_states,
							})
						).unwrap()
					},
					ServerMessage::PlayerJoin(pstate) => {
						to_string(&json!({
							"t": "PlayerJoin",
							"c": pstate.encode(pstate.public_id == public_id),
						})).unwrap()
					},
					_ => String::new()
				}
			},
			_ => to_string(msg).unwrap()
		};
		ch.send(Ok(Message::text(serialized_msg)));
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
	for (private_id, client) in clients_readlock.iter(){
		let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
		client.transmit(msg, Some(public_id)).await;
	}
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

	let clr = clients.read().await;
	let sender_state = match clr.get(private_id) {
		Some(v) => &v.state,
		None => {
			eprintln!("Can't find sender in clients: {}", private_id);
			return;
		}
	};

	let health = sender_state.read().await.clone().health;
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
			spawn_from_prev(&mut *sender_state.write().await); //reset health and position
			broadcast(
				&ServerMessage::PlayerJoin(sender_state.read().await.clone()),
				&clr
			).await; //broadcast playerjoin
		},
		ClientMessage::MotionUpdate { motion } => {
			if sender_state.read().await.clone().motion.direction == motion { //nothing changed
				eprintln!("modded client detected (redundant motion update)");
				return;
			}
			let (nx, ny) = live_pos(&sender_state.read().await.clone());
			{
				let mut writeable = sender_state.write().await;
				writeable.x = nx;
				writeable.y = ny;
				writeable.motion = MotionStart{
					direction: motion,
					time: Instant::now()
				};
			} //This makes writable out of scope, so the write lock is released
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

			{
				let mut writeable = sender_state.write().await;
				writeable.rotation = nr;
				writeable.rotation_motion = RotationStart{
					direction: direction,
					time: Instant::now()
				};
			} //This puts 'writeable' out of scope, freeing the resource

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

			//gpt-4 did this
			// Use futures::future::join_all to wait for all tasks to complete.
			let players_futures: Vec<_> = clr.iter()
					.map(|(_, value)| value.state.read())
					.collect();

			let players: Vec<_> = join_all(players_futures).await
					.into_iter()
					.map(|lock| lock.clone())
					.collect();

			// Send the response to the client.
			if let Some(client) = clr.get(private_id) {
				let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
				client.transmit(&ServerMessage::GameState(players), Some(public_id)).await;
			} else {
					eprintln!("Can't find client")
			}
		},
		ClientMessage::ChangeSlot { slot } => { //TODO consider also changing trigger_pressed to false both on client and server when changeslot
			if sender_state.read().await.clone().inventory.selection != slot
				{ sender_state.write().await.inventory.selection = slot; }
		},
		ClientMessage::TrigUpdate { pressed } => {
			let cl = sender_state.read().await.clone();

			//This would require a modded client
			if cl.trigger_pressed == pressed {
				eprintln!("trigger press identical");
				return;
			}

			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			sender_state.write().await.trigger_pressed = pressed;
			let weapon_selected = cl.inventory.weapons.get(
				&cl.inventory.selection
			).unwrap().clone();

			//check if there is even ammo
			if weapon_selected.ammo <= 0 {
				return;
			}

			//check if ammo needs to be decremented
			match (weapon_selected.weptype.clone(), pressed) {
				(WeaponType::Pistol, false) => {},
				(WeaponType::Grenade {press_time: _}, true) => {},
				_ => {
					let mut writeable = sender_state.write().await;
					writeable.inventory.weapons.get_mut(
							&cl.inventory.selection
					).unwrap().ammo -= 1;
				}
			};

			//check if client needs to know
			match (weapon_selected.weptype.clone(), pressed) {
				(WeaponType::Pistol, false) => {},
				_ => {
					broadcast(
						&ServerMessage::TrigUpdate {
							by: public_id,
							weptype: weapon_selected.weptype.clone(),
							pressed: pressed,
						},
						&clr
					).await;				
				}
			};

			//Update healths
			match weapon_selected.weptype {
				//TODO since we don't care if pistol released, we should make the client not even send it
				WeaponType::Pistol => {
					if !pressed {
						return;
					}
					let rr = live_rot(&cl);
					let (sx, sy) = live_pos(&cl);

					//boring linear search
					for (key, value) in clr.iter() { //TODO try using for_each
						if key == private_id{ //Can't shoot yourself
							continue;
						}

						let playerstate = value.state.read().await.clone();

						//ignore dead players
						if playerstate.health <= 0f32 {
							continue;
						}
						let (px, py) = live_pos(&playerstate);
						let hit = line_circle_intersect(sx, sy, px, py, rr);
						if !hit{
							continue;
						}

						let new_health = {
							let mut writeable = value.state.write().await;
							writeable.health -= 10f32; //hard coded pistol damage to 10
							//playerstate.health -= 10f32; //update the copy too
							writeable.health
						};

						broadcast(
							&ServerMessage::HealthUpdate {
								value: new_health,
								from: format!("{:x}", xxh3_64(key.as_bytes()))
							},
							//clients
							&clr
						).await;
						
						break; //important! you can only hit one player at one time
					};
				},
				WeaponType::Grenade { press_time: _ } => {
					if pressed {
						
					}
				}
			}
		}
	}
}
