use crate::shared_gameobjects::ShopItemId;
use crate::trajectory::*;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub enum LootContent {
	Cash(u32),
	PistolAmmo(u32),
	Health(u8),
	SpeedBoost,
}

#[derive(Serialize, Debug, Clone)]
pub struct LootObject {
	pub x: f32,
	pub y: f32,
	pub loot: LootContent,
}

#[derive(Serialize, Debug, Clone)]
pub struct Color {
	pub r: i32,
	pub g: i32,
	pub b: i32,
}

#[derive(Serialize, Debug, Clone)]
pub struct Weapon {
	pub weptype: WeaponType,
	pub ammo: u32,
}

#[derive(Debug, Clone)]
pub enum WeaponType {
	Pistol,
	Grenade { press_time: f32 },
}

impl Serialize for WeaponType {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		match self {
			WeaponType::Pistol => serializer.serialize_str("Pistol"),
			WeaponType::Grenade { press_time: _ } => serializer.serialize_str("Grenade"),
		}
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct Inventory {
	pub selection: u8,
	pub weapons: HashMap<u8, Weapon>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LootDrop {
	pub uuid: String,
	pub object: LootObject,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerState {
	pub name: String,
	pub id: String,
	pub cash: u32,
	pub fuel: u32,
	pub color: Color,
	pub inventory: Inventory,
	pub trigger_pressed: bool,
	#[serde(skip_serializing)]
	pub trajectory: Trajectory,
}

impl PlayerState {
	pub fn encode_other(&self) -> Value {
		//TODO consider implementing live() in Trajectory - an immutable version of reset() and use that instead
		return json!({
			"name": &self.name,
			"id": &self.id,
			"color": &self.color,
			"trajectory": &self.trajectory.to_b64(),
		});
	}

	pub fn encode(&self, as_self: bool) -> Value {
		if !as_self {
			return self.encode_other();
		}
		let mut result = self.encode_other();
		let additional = json!({
			"inventory": &self.inventory,
			"cash": &self.cash,
		});
		result
			.as_object_mut()
			.unwrap()
			.extend(additional.as_object().unwrap().clone());
		return result;
	}
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Victim {
	pub id: String,
	pub hash: String,
	pub time: u64,

	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(skip_deserializing)]
	pub loot: Option<LootDrop>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShootInfo {
	//details about when the shooter shot
	pub at: String,
	pub stime: u64,

	//These are for when the message comes from the server
	#[serde(skip_serializing_if = "Option::is_none")]
	pub shooter: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub victim: Option<Victim>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ClientMessage {
	Ping,
	AckPong,
	TrajectoryUpdate {
		change: UpdateTypeWrapper,
		at: String,
		time: u64,
	},
	ChangeSlot {
		slot: u8,
	},
	ClaimLoot {
		loot_id: String,
	},
	Correct(String),
	Shoot(ShootInfo),
	StateQuery,
	Spawn,
}

#[derive(Serialize, Debug)]
#[serde(tag = "t", content = "c")]
pub enum ServerMessage {
	Pong(u64),
	PlayerJoin(PlayerState),
	PlayerLeave(String),
	HealthUpdate(u8),
	GameState {
		pstates: Vec<PlayerState>,
		worldloot: HashMap<String, LootObject>,
	},
	TrajectoryUpdate {
		change: UpdateTypeWrapper,
		time: u64,
		at: String,
		from: String,
	},
	Shoot(ShootInfo),
	PlayerDeath {
		loot: Option<LootDrop>,
		from: String,
	},
	LootCollected {
		loot_id: String,
		collector: String,
	},
	Correct {
		id: String,
		tr: String,
	},
	LootReject(String),
}
