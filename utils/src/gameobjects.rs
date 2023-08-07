use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::trajectory::*;

#[derive(Serialize, Debug, Clone)]
pub enum LootContent{
	Cash(u32),
	PistolAmmo(u32),
	SpeedBoost,
}

#[derive(Serialize, Debug, Clone)]
pub struct LootObject{
	pub x: f32,
	pub y: f32,
	pub loot: LootContent,
}

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
	Grenade {press_time: f32}
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
pub struct LootDrop{
	pub uuid: String,
	pub object: LootObject,
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
		let pos = &self.trajectory.pos;
		let vel = &self.trajectory.vel;
		let spin = &self.trajectory.spin;
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage{
	Ping,
	AckPong,
	Propel,
	PropelStop,
	Rotation {dir: i8},
	ChangeSlot {slot: u8},
	TrigUpdate {pressed: bool},
	ClaimLoot {loot_id: String},
	StateQuery,
	Spawn,
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage{
	Pong,
	PlayerJoin(PlayerState),
	PlayerLeave(String),
	HealthUpdate(f32),
	GameState{
		pstates: Vec<PlayerState>,
		worldloot: HashMap<String, LootObject>,
		bodies: Vec<Body>,
	},
	PropelUpdate { propel: bool, pos: Vector, vel: Vector, from: String },
	RotationUpdate { direction: i8, spin: f32, from: String },
	TrigUpdate {by: String, weptype: WeaponType, pressed: bool },
	PlayerDeath {loot: Option<LootDrop>, from: String },
	LootCollected {loot_id: String, collector: String },
	LootReject(String),
}
