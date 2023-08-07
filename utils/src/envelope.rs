macro_rules! server_deserialize_fn {
	($name:ident, $type:ty) => {
		pub fn $name(bytes: &[u8]) -> Result<$type, JsValue> {
			deserialize(bytes).map_err(|e| e.to_string().into())
		}
	};
}

macro_rules! client_deserialize_fn {
	($name:ident, $type:ty) => {
		#[wasm_bindgen]
		pub fn $name(bytes: &[u8]) -> Result<$type, JsValue> {
			deserialize(bytes).map_err(|e| e.to_string().into())
		}
	};
}

macro_rules! server_serialize_fn {
	($name:ident, $type:ty, $literal:expr) => {
		pub fn $name(item: &$type) -> Result<ServerEnvelope, Box<bincode::ErrorKind>> {
			let payload = serialize(&item)?;
			Ok(
				ServerEnvelope {
					mtype: $literal,
					m: payload,
				}
			)
		}
	};
}

macro_rules! client_serialize_fn {
	($name:ident, $type:ty, $literal:expr) => {
		#[wasm_bindgen]
		pub fn $name(item: &$type) -> Result<ClientEnvelope, Box<bincode::ErrorKind>> {
			let payload = serialize(&item)?;
			Ok(
				ClientEnvelope {
					mtype: $literal,
					m: payload,
				}
			)
		}
	};
}

use wasm_bindgen::prelude::*;
use serde::{Deserialize,Serialize};
use bincode::{serialize, deserialize};
use crate::gameobjects::*;
use crate::trajectory::*;
use crate::servermessage;

pub enum ClientMessage {
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

pub enum ServerMessage {
	State,
	Propel,
	Rotation,
	Trigger,
	Death,
	Collect,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ServerEnvelope {
	mtype: ServerMessage,
	m: Vec<u8>,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct ClientEnvelope {
	mtype: ClientMessage,
	m: Vec<u8>,
}

client_deserialize_fn(deserialize_envelope, ServerEnvelope);
client_deserialize_fn(deserialize_gamestate, servermessage::GameState);
client_deserialize_fn(deserialize_propelupdate, servermessage::PropelUpdate);
client_deserialize_fn(deserialize_rotationupdate, servermessage::RotationUpdate);
client_deserialize_fn(deserialize_trigupdate, servermessage::TrigUpdate);
client_deserialize_fn(deserialize_lootcollected, servermessage::LootCollected);

client_serialize_fn(serialize_ping, )

server_serialize_fn(serialize_gamestate, servermessage::GameState, ServerMessage::State);
server_serialize_fn(serialize_propelupdate, servermessage::PropelUpdate, ServerMessage::Propel);
server_serialize_fn(serialize_rotateupdate, servermessage::RotationUpdate, ServerMessage::Rotation);
server_serialize_fn(serialize_trigupdate, servermessage::TrigUpdate, ServerMessage::Trigger);
server_serialize_fn(serialize_playerdeath, servermessage::PlayerDeath, ServerMessage::Death);
server_serialize_fn(serialize_lootcollected, servermessage::LootCollected, ServerMessage::Collect);

server_deserialize_fn(deserialize_clientenvelope, ClientEnvelope);
server_deserialize_fn(deserialize_i8, i8);
server_deserialize_fn(deserialize_u8, u8);
server_deserialize_fn(deserialize_bool, bool);
server_deserialize_fn(deserialize_string, String);
