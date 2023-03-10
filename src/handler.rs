use crate::{ws, Client, Clients, Result};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use xxhash_rust::xxh3::xxh3_64;
use warp::{http::StatusCode, reply::json, Reply};
use std::fs;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use tokio::sync::RwLock;
use rand::Rng;

use crate::PlayerState;
use crate::game::Color;
use crate::game::MotionStart;
use crate::game::RotationStart;
use crate::game::PlayerMotion;
use crate::game::PlayerRotation;
use crate::game::Inventory;
use crate::game::Weapon;
use crate::game::WeaponType;

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
	public: String,
	private: String,
}

#[derive(Deserialize)]
pub struct UserSelections {
	nick: String,
	color: String
}

pub async fn register_handler(body: Value, clients: Clients) -> Result<impl Reply> {
	println!("{}", body);
	let selection_result = serde_json::from_value::<UserSelections>(body);
	match selection_result {
		Ok(selections) => {
			if selections.nick.len() > 10 {
				return Ok(json(&"meow"))
			}

			let private_uuid = Uuid::new_v4().as_simple().to_string();
			let public_id = format!("{:x}", xxh3_64(private_uuid.as_bytes()));
			println!("Registering client {}", public_id);
			register_client(
				private_uuid.clone(),
				public_id.clone(),
				selections,
				clients,
			).await;
			Ok(json(&RegisterResponse {
				public: public_id,
				private: private_uuid,
			}))
		},
		Err(_) => Ok(json(&"meow")) //TODO write this in an acceptable manner
	}
}

pub async fn serve_page() -> Result<impl Reply> {
	let html = fs::read_to_string("client/site.html").unwrap();
	Ok(warp::reply::html(html))
}

async fn register_client(private_id: String, public_id: String, selections: UserSelections, clients: Clients) {
	clients.write().await.insert(
		private_id,
		Client {
			state: Arc::new(RwLock::new(
				PlayerState{
					name: selections.nick,
					public_id: public_id,
					x: rand::thread_rng().gen_range(-300f32..300f32),
					y: rand::thread_rng().gen_range(-300f32..300f32),
					rotation: 0.0,
					health: 100.0,
					color: match selections.color.as_str() {
						"red" => Color{r:255,g:0,b:0},
						"orange" => Color{r:255,g:165,b:0},
						"yellow" => Color{r:255,g:255,b:0},
						"green" => Color{r:0,g:255,b:0},
						"blue" => Color{r:0,g:0,b:255},
						_ => Color{r:255,g:255,b:255}
					},
					inventory: Inventory{
						selection: 0,
						weapons: HashMap::from([
							(0, Weapon{ weptype: WeaponType::Pistol, ammo: 10 }),
							(1, Weapon{ weptype: WeaponType::FlameThrower, ammo: 100 }),
						])
					},
					trigger_pressed: false,
					motion: MotionStart{direction: PlayerMotion::Stopped, time: Instant::now()},
					rotation_motion: RotationStart{direction: PlayerRotation::Stopped, time: Instant::now()}
				}
			)),
			sender: None,
		},
	);
}

pub async fn unregister_handler(private_id: String, clients: Clients) -> Result<impl Reply> {
	clients.write().await.remove(&private_id);
	Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, private_id: String, clients: Clients) -> Result<impl Reply> {
	let client = clients.read().await.get(&private_id).cloned();
	match client {
		Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, private_id, clients, c))),
		None => Err(warp::reject::not_found()),
	}
}

pub async fn health_handler() -> Result<impl Reply> {
	Ok(StatusCode::OK)
}
