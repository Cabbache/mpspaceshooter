use std::collections::HashMap;
use std::f32::consts::{PI};
use std::time::Instant;
use std::error::Error;

use futures::future::join_all;
use num_traits::{Pow};
//use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{from_str, json, to_string, Value};
use warp::ws::Message;
use xxhash_rust::xxh3::xxh3_64;
use uuid::Uuid;
use rand::Rng;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::Clients;
use crate::Client;
use crate::WorldLoot;
use crate::handler::spawn_from_prev;

const RADIANS_PER_SECOND: f32 = PI; //player rotation speed
const PLAYER_RADIUS: f32 = 25.0; //players have circular hitbox
const LOOT_RADIUS: f32 = 25.0; //players must be within this distance to claim
const PISTOL_REACH: f32 = 500.0; //players have circular hitbox
const ACCELERATION: f32 = 200.0; //player acceleration
const PROPEL_DIRECTION: f32 = -PI/2.0;

#[derive(Serialize, Debug, Clone)]
pub enum LootContent{
	Cash(u32),
	PistolAmmo(u32),
	SpeedBoost,
}

#[derive(Serialize, Debug, Clone)]
pub struct LootObject{
	x: f32,
	y: f32,
	loot: LootContent,
}

//Arc<RwLock<HashMap<String, LootObject>>>

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
pub struct Vector{
	pub x: f32,
	pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Trajectory{
	pub propelling: bool,
	pub pos: Vector,
	pub vel: Vector,
	pub spin: f32,
	pub spin_direction: PlayerRotation,
	pub time: Instant,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerState {
	pub name: String,
	pub public_id: String,
	pub health: f32,
	pub cash: u32,
	pub fuel: u32,
	pub color: Color,
	pub inventory: Inventory,
	pub trigger_pressed: bool,
	#[serde(skip_serializing)]
	pub trajectory: Trajectory,
}

impl PlayerState {
	pub fn encode_other(&self) -> Value{
		//TODO consider implementing live() in Trajectory - an immutable version of reset() and use that instead
		let pos = &self.trajectory.live_pos();
		let vel = &self.trajectory.live_vel();
		let spin = &self.trajectory.live_rot();
		return json!({
			"name": &self.name,
			"public_id": &self.public_id,
			"color": &self.color,
			"propelling": &self.trajectory.propelling,
			"pos": &pos,
			"vel": &vel,
			"spin": &spin,
			"spinDir": &self.trajectory.spin_direction,
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
			"cash": &self.cash,
		});
		result
		.as_object_mut()
		.unwrap()
		.extend(
			additional
			.as_object().
			unwrap()
			.clone()
		);
		return result;
	}
}

impl Client {
	pub async fn transmit(&self, msg: &ServerMessage, public_id: Option<String>) -> Result<(), Box<dyn Error>> {
		if let Some(ch) = self.sender.as_ref() {
			let public_id = match public_id {
				Some(id) => id, 
				None => self.state.read().await.public_id.clone(),
			};
			let serialized_msg = match msg {
				ServerMessage::PlayerJoin(_) |
				ServerMessage::GameState{pstates: _, worldloot: _} => {
				//ServerMessage::LootCollected{loot_id: _, collector: _} => {
					match msg {
						ServerMessage::GameState{ pstates, worldloot } => {
							let encoded_states: Vec<Value> = pstates.iter().map(|state| {
								state.encode(public_id == state.public_id)
							}).collect();
							to_string(&json!({
								"t": "GameState",
								"c": {
									"players": encoded_states,
									"loot": worldloot,
								}
							}))?
						},
						ServerMessage::PlayerJoin(pstate) => {
							to_string(&json!({
								"t": "PlayerJoin",
								"c": pstate.encode(pstate.public_id == public_id),
							}))?
						},
//						ServerMessage::LootCollected{ loot_id, collector } => {
//							let mut content: Value = json!({
//								"loot_id": loot_id
//							});
//							if *collector == public_id {
//								content["receiver"] = json!(1);
//							}
//							to_string(&json!({
//								"t": "LootCollected",
//								"c": content,
//							}))?
//						}
						_ => String::new()
					}
				},  
				_ => to_string(msg)?
			};
			ch.send(Ok(Message::text(serialized_msg))).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
		}
		Ok(())
	}
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum PlayerRotation {
	AntiClockwise,
	Clockwise,
	Stopped
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage{
	Propel,
	PropelStop,
	Rotation {dir: PlayerRotation},
	ChangeSlot {slot: u8},
	TrigUpdate {pressed: bool},
	ClaimLoot {loot_id: String},
	StateQuery,
	Spawn,
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage{
	PlayerJoin(PlayerState),
	PlayerLeave(String),
	HealthUpdate(f32),
	GameState{ pstates: Vec<PlayerState>, worldloot: HashMap<String, LootObject> },
	PropelUpdate { propel: bool, pos: Vector, vel: Vector, from: String },
	RotationUpdate { direction: PlayerRotation, spin: f32, from: String },
	TrigUpdate {by: String, weptype: WeaponType, pressed: bool },
	PlayerDeath {loot: LootObject, loot_uuid: String, from: String },
	LootCollected {loot_id: String, collector: String },
	LootReject(String),
}

pub async fn broadcast(msg: &ServerMessage, clients_readlock: &tokio::sync::RwLockReadGuard<'_, HashMap<std::string::String, Client>>){
	for (private_id, client) in clients_readlock.iter(){
		let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
		if let Err(e) = client.transmit(msg, Some(public_id)).await {
			eprintln!("Error transmitting message: {}", e);
		}
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

//TODO make cos/sin faster by storing the results
impl Trajectory {
	fn reset(&mut self){
		let seconds = (Instant::now() - self.time).as_secs_f32();
		let base = Vector{
			x: self.pos.x + seconds*self.vel.x,
			y: self.pos.y + seconds*self.vel.y,
		};
		match self.propelling {
			true => {
				match self.spin_direction {
					PlayerRotation::Stopped => {
						self.pos = Vector{
							x: base.x + seconds*seconds*(self.spin + PROPEL_DIRECTION).cos()*ACCELERATION/2.0,
							y: base.y + seconds*seconds*(self.spin + PROPEL_DIRECTION).sin()*ACCELERATION/2.0,
						};
						self.vel = Vector{
							x: self.vel.x + seconds*(self.spin + PROPEL_DIRECTION).cos()*ACCELERATION,
							y: self.vel.y + seconds*(self.spin + PROPEL_DIRECTION).sin()*ACCELERATION,
						};
					},
					_ => {
						let speed = match self.spin_direction {
							PlayerRotation::Clockwise => 1.0,
							PlayerRotation::AntiClockwise => -1.0,
							_ => 0.0,
						} * RADIANS_PER_SECOND;
						self.pos = Vector{
							x: base.x + ACCELERATION*(((self.spin + PROPEL_DIRECTION).cos() - (speed*seconds + (self.spin + PROPEL_DIRECTION)).cos())/speed - (self.spin + PROPEL_DIRECTION).sin()*seconds) / speed,
							y: base.y + ACCELERATION*((self.spin + PROPEL_DIRECTION).cos()*seconds - ((speed*seconds + (self.spin + PROPEL_DIRECTION)).sin() - (self.spin + PROPEL_DIRECTION).sin())/speed) / speed,
						};
						self.vel = Vector{
							x: self.vel.x + ACCELERATION*((speed*seconds + (self.spin + PROPEL_DIRECTION)).sin() - (self.spin + PROPEL_DIRECTION).sin()) / speed,
							y: self.vel.y + ACCELERATION*((self.spin + PROPEL_DIRECTION).cos() - (speed*seconds + (self.spin + PROPEL_DIRECTION)).cos()) / speed,
						};
						self.spin = self.spin + speed*seconds;
					}
				}
			},
			false => {
				self.pos = base;
				self.spin = match self.spin_direction {
					PlayerRotation::Clockwise => 1.0,
					PlayerRotation::AntiClockwise => -1.0,
					_ => 0.0,
				} * RADIANS_PER_SECOND * seconds + self.spin;
			}
		}
		self.time = Instant::now();
	}

	pub fn live_vel(&self) -> Vector {
		match self.propelling {
			false => self.vel.clone(),
			true => {
				let seconds = (Instant::now() - self.time).as_secs_f32();
				match self.spin_direction {
					PlayerRotation::Stopped => Vector{
						x: self.vel.x + (self.spin + PROPEL_DIRECTION).cos()*ACCELERATION*seconds,
						y: self.vel.y + (self.spin + PROPEL_DIRECTION).sin()*ACCELERATION*seconds,
					},
					_ => {
						let speed = match self.spin_direction {
							PlayerRotation::Clockwise => 1.0,
							PlayerRotation::AntiClockwise => -1.0,
							_ => 0.0,
						} * RADIANS_PER_SECOND;

						let d_vel = Vector{
							x: ACCELERATION*(speed*seconds + (self.spin + PROPEL_DIRECTION)).sin() / speed,
							y: ACCELERATION*(speed*seconds + (self.spin + PROPEL_DIRECTION)).cos() / -speed,
						};
						
						Vector{
							x: self.vel.x + d_vel.x,
							y: self.vel.y + d_vel.y,
						}
					}
				}
			}
		}
	}

	pub fn live_pos(&self) -> Vector {
		let seconds = (Instant::now() - self.time).as_secs_f32();
		let base = Vector{
			x: self.pos.x + seconds*self.vel.x,
			y: self.pos.y + seconds*self.vel.y,
		};
		match self.propelling {
			true => {
				match self.spin_direction {
					PlayerRotation::Stopped => {
						Vector{
							x: base.x + seconds*seconds*(self.spin + PROPEL_DIRECTION).cos()*ACCELERATION/2.0,
							y: base.y + seconds*seconds*(self.spin + PROPEL_DIRECTION).sin()*ACCELERATION/2.0,
						}
					},
					_ => {
						let speed = match self.spin_direction {
							PlayerRotation::Clockwise => 1.0,
							PlayerRotation::AntiClockwise => -1.0,
							_ => 0.0,
						} * RADIANS_PER_SECOND;

						//cos(spin + speed*t)*acceleration <- x acceleration function
						//sin(spin + speed*t)*acceleration <- y acceleration function
						
						//Integral of above (the current change in velocity)
						//ACCELERATION*sin(speed*t + self.spin) / speed
						//ACCELERATION*cos(speed*t + self.spin) / -speed
						//let dVel = Vector{
						//	x: ACCELERATION*(speed*seconds + self.spin).sin() / speed
						//	y: ACCELERATION*(speed*seconds + self.spin).cos() / -speed
						//};
						
						//Integral of above (the current change in position)
						let d_pos = Vector{
							x: -ACCELERATION*(speed*seconds + (self.spin + PROPEL_DIRECTION)).cos() / (speed*speed),
							y: -ACCELERATION*(speed*seconds + (self.spin + PROPEL_DIRECTION)).sin() / (speed*speed),
						};

						Vector{
							x: base.x + d_pos.x,
							y: base.y + d_pos.y,
						}
					}
				}
			},
			false => base
		}
	}

	pub fn live_rot(&self) -> f32 {
		let seconds = (Instant::now() - self.time).as_secs_f32();
		(match self.spin_direction {
			PlayerRotation::Stopped => 0.0,
			PlayerRotation::AntiClockwise => -seconds,
			PlayerRotation::Clockwise => seconds,
		}) * RADIANS_PER_SECOND + self.spin
	}

	pub fn update_rotation(&mut self, new_direction: &PlayerRotation){
		self.reset();
		self.spin_direction = *new_direction;
	}

	pub fn update_propulsion(&mut self, on: bool){
		self.reset();
		self.propelling = on;
	}
}

//TODO capture the current time and pass it to live_pos and live_rot
//This will improve accuracy due to lock acquire times
pub async fn handle_game_message(private_id: &str, message: &str, clients: &Clients, world_loot: &WorldLoot) -> Result<(), Box<dyn Error>>{
	let message: ClientMessage = match from_str(message) {
		Ok(v) => v,
		Err(m) => {
			eprintln!("Can't deserialize message: {}", m);
			return Ok(());
		}
	};

	let clr = clients.read().await;
	let sender_state = match clr.get(private_id) {
		Some(v) => &v.state,
		None => {
			eprintln!("Can't find sender in clients: {}", private_id);
			return Ok(());
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
		return Ok(());
	}

	match message {
		ClientMessage::Spawn => {
			spawn_from_prev(&mut *sender_state.write().await); //reset health and position
			broadcast(
				&ServerMessage::PlayerJoin(sender_state.read().await.clone()),
				&clr
			).await; //broadcast playerjoin
		},
		ClientMessage::Propel |
		ClientMessage::PropelStop => {
			let is_propel = message == ClientMessage::Propel;
			if sender_state.read().await.clone().trajectory.propelling == is_propel{ //nothing changed
				eprintln!("modded client detected (redundant propel update)");
				return Ok(());
			}
			let new_trajectory = {
				let mut writeable = sender_state.write().await;
				println!("before {:?}", writeable.trajectory);
				writeable.trajectory.update_propulsion(is_propel);
				writeable.trajectory.clone()
			};
			println!("after {:?}", new_trajectory);
			broadcast(
				&ServerMessage::PropelUpdate{
					propel: is_propel,
					pos: new_trajectory.pos,
					vel: new_trajectory.vel,
					from: format!("{:x}", xxh3_64(private_id.as_bytes())),
				},
				&clr,
			).await;
		},
		ClientMessage::Rotation { dir } => {
			if sender_state.read().await.clone().trajectory.spin_direction == dir {
				eprintln!("modded client detected (redundant propel update)");
				return Ok(());
			}
			let new_trajectory = {
				let mut writeable = sender_state.write().await;
				writeable.trajectory.update_rotation(&dir);
				&writeable.trajectory.clone()
			};
			broadcast(
				&ServerMessage::RotationUpdate {
					direction: dir,
					spin: new_trajectory.spin,
					from: format!("{:x}", xxh3_64(private_id.as_bytes())),
				},
				&clr
			).await;
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
				.filter_map(|lock| {
					let clone = lock.clone();
					match clone.health > 0.0 { //to only send the living ones
						true => Some(clone),
						false => None
					}
				})
				.collect();

			//Send the response to the client.
			if let Some(client) = clr.get(private_id) {
				let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
				client.transmit(
					&ServerMessage::GameState{
						pstates: players,
						worldloot: world_loot.read().await.clone()
					},
					Some(public_id)
				).await?;
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
				return Ok(());
			}

			let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
			sender_state.write().await.trigger_pressed = pressed;
			let weapon_selected = cl.inventory.weapons.get(
				&cl.inventory.selection
			).unwrap().clone();

			//check if there is even ammo
			if weapon_selected.ammo <= 0 {
				return Ok(());
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
						return Ok(());
					}
					let rr = cl.trajectory.live_rot();
					let ss = cl.trajectory.live_pos();

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
						let pp = playerstate.trajectory.live_pos();
						let hit = line_circle_intersect(ss.x, ss.y, pp.x, pp.y, rr);
						if !hit{
							continue;
						}

						let new_health = {
							let mut writeable = value.state.write().await;
							writeable.health -= 10f32; //hard coded pistol damage to 10
							//playerstate.health -= 10f32; //update the copy too
							writeable.health
						};

						if new_health > 0f32 {
							value.transmit(
								&ServerMessage::HealthUpdate(new_health),
								Some(playerstate.public_id)
							).await?; //tell the player that lost the health their new health
						} else {
							let mut rng = StdRng::from_entropy();
							let dropped_loot = LootObject{
								x: pp.x,
								y: pp.y,
								loot: match rng.gen_range(0..101){
									0..=33 => LootContent::Cash(playerstate.cash / 2),
									34..=67 => LootContent::PistolAmmo(15),
									_ => LootContent::SpeedBoost,
								}
							};
							let dropped_loot_uuid = Uuid::new_v4().as_simple().to_string();
							world_loot.write().await.insert(
								dropped_loot_uuid.clone(),
								dropped_loot.clone(),
							);
							broadcast( //tell everyone that the player died and what loot they dropped
								&ServerMessage::PlayerDeath{
									loot: dropped_loot,
									loot_uuid: dropped_loot_uuid,
									from: playerstate.public_id,
								},
								&clr
							).await;
						}
						break; //important! you can only hit one player at one time
					};
				},
				WeaponType::Grenade { press_time: _ } => {
					if pressed {
						
					}
				}
			}
		},
		ClientMessage::ClaimLoot { loot_id } => {
			let loot_thing = {
				let loot_lock = world_loot.read().await;
				loot_lock.get(&loot_id).cloned()
			};
			match loot_thing {
				Some(loot_obj) => {
					let pp = &sender_state.read().await.clone().trajectory.live_pos();
					if (pp.y - loot_obj.y).pow(2) + (pp.x - loot_obj.x).pow(2) > LOOT_RADIUS.pow(2){
						if let Some(client) = clr.get(private_id){
							client.transmit(&ServerMessage::LootReject(loot_id), Some(format!("{:x}", xxh3_64(private_id.as_bytes())))).await?;
						} else {
							eprintln!("Weird, did not find client {} in clr", private_id);
						}
						return Err("Too far for loot claim".into());
					}

					{
						//make sure to acquire both locks before proceeding
						let mut pstate_writer = sender_state.write().await;
						let mut world_loot_writer = world_loot.write().await;

						world_loot_writer.remove(&loot_id);
						match loot_obj.loot {
							LootContent::Cash(amount) => pstate_writer.cash += amount,
							LootContent::PistolAmmo(amount) => {
								let selection = pstate_writer.inventory.selection;
								if let Some(weapon) = pstate_writer.inventory.weapons.get_mut(&selection) {
									weapon.ammo += amount;
								}
							},
							LootContent::SpeedBoost => {
							}
						}
					}//locks are released
					println!("locks released");

					let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
					broadcast(
						&ServerMessage::LootCollected { loot_id: loot_id, collector: public_id },
						&clr
					).await; //broadcast that loot was collected
				},
				None => {
					return Err(format!("Can't find requested lootobject: {}", loot_id).into());
				}
			};
		}
	}
	Ok(())
}
