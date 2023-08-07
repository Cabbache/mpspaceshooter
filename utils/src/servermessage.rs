use serde::{Deserialize, Serialize, Serializer};
use crage::gameobjects::*;

//#[derive(Serialize, Debug)]
//#[serde(tag = "t", content = "c")]
//pub enum ServerMessage{
//	Pong,
//	PlayerJoin(PlayerState),
//	PlayerLeave(String),
//	HealthUpdate(f32),
//	LootReject(String),
//}

pub struct GameState {
	pub pstates: Vec<PlayerState>,
	pub worldloot: HashMap<String, LootObject>,
	pub bodies: Vec<Body>,
}

pub struct PropelUpdate {
	pub propel: bool,
	pub pos: Vector,
	pub vel: Vector,
	pub from: String,
}

pub struct RotationUpdate {
	pub direction: i8,
	pub spin: f32,
	pub from: String,
}

pub struct TrigUpdate {
	pub by: String,
	pub weptype: WeaponType,
	pub pressed: bool,
}

pub struct PlayerDeath {
	pub loot: Option<LootDrop>,
	pub from: String,
}

pub struct LootCollected {
	pub loot_id: String,
	pub collector: String,
}
