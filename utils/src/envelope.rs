use wasm_bindgen::prelude::*;
use serde::{Deserialize,Serialize};
use bincode::{serialize, deserialize};
use crate::trajectory::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Envelope {
	mtype: String,
	m: Vec<u8>,
}

#[wasm_bindgen]
pub fn deserialize_envelope(bytes: &[u8]) -> Result<Envelope, JsValue> {
	deserialize(bytes).map_err(|e| e.to_string().into())
}

#[wasm_bindgen]
pub fn deserialize_trajectory(bytes: &[u8]) -> Result<Trajectory, JsValue> {
	deserialize(bytes).map_err(|e| e.to_string().into())
}

pub fn serialize_trajectory(trajectory: &Trajectory) -> Result<Envelope, Box<bincode::ErrorKind>> {
	let payload = serialize(&trajectory)?;
	Ok(Envelope {
		mtype: "trajectory".to_string(),
		m: payload,
	})
}
