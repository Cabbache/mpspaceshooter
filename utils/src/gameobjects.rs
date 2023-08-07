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
	pub trajectory: Trajectory,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerStateOther {
	pub name: String,
	pub public_id: String,
	pub fuel: u32,
	pub color: Color,
	pub trigger_pressed: bool,
	pub trajectory: Trajectory,
}

impl From<PlayerState> for PlayerStateOther {
	fn from(player_state: PlayerState) -> Self {
		PlayerStateOther {
			name: player_state.name,
			public_id: player_state.public_id,
			fuel: player_state.fuel,
			color: player_state.color,
			trigger_pressed: player_state.trigger_pressed,
			trajectory: player_state.trajectory,
		}
	}
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(tag = "t", content = "c")]

