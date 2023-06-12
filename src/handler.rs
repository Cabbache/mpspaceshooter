use crate::{ws, Client, Clients, Result};
use serde::{Serialize, Deserialize};
use warp::{http::StatusCode, reply::json, Reply};
use rand_distr::{Normal, Distribution};
use xxhash_rust::xxh3::xxh3_64;
use uuid::Uuid;
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::fs;

use crate::PlayerState;
use crate::WorldLoot;
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
			if selections.nick.len() > 24 {
				return Ok(json(&"meow"))
			}

			let private_uuid = Uuid::new_v4().as_simple().to_string();
			let public_id = format!("{:x}", xxh3_64(private_uuid.as_bytes()));
			println!("Registering client {}", private_uuid);
			register_client(
				private_uuid.clone(),
				&public_id,
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

fn default_state() -> PlayerState {
	let normal = Normal::new(0.0, 100.0).unwrap();
	let pos_x = normal.sample(&mut rand::thread_rng());
	let pos_y = normal.sample(&mut rand::thread_rng());
	println!("x: {}", pos_x);
	println!("y: {}", pos_y);
	return PlayerState {
		name: "".to_string(),
		public_id: "".to_string(),
		x: pos_x,
		y: pos_y,
		rotation: 0.0,
		health: 100.0,
		cash: 20,
		color: Color{r:255,g:255,b:255},
		inventory: Inventory{
			selection: 0,
			weapons: HashMap::from([
				(0, Weapon{ weptype: WeaponType::Pistol, ammo: 50 }),
				(1, Weapon{ weptype: WeaponType::Grenade {press_time: Instant::now()}, ammo: 2 }),
			])
		},
		trigger_pressed: false,
		motion: MotionStart{direction: PlayerMotion::Stopped, time: Instant::now()},
		rotation_motion: RotationStart{direction: PlayerRotation::Stopped, time: Instant::now()}	
	}
}

pub fn spawn_with_select(selections: &UserSelections, public_id: &String) -> PlayerState {
	PlayerState{
		name: selections.nick.clone(),
		public_id: public_id.clone(),
		color: match selections.color.as_str() {
			"red" => Color{r:255,g:0,b:0},
			"orange" => Color{r:255,g:165,b:0},
			"yellow" => Color{r:255,g:255,b:0},
			"green" => Color{r:0,g:255,b:0},
			"blue" => Color{r:0,g:0,b:255},
			_ => Color{r:255,g:255,b:255}
		},
		..default_state()
	}
}

pub fn spawn_from_prev(prev_state: &mut PlayerState) {
	*prev_state = PlayerState {
		name: prev_state.name.clone(),
		public_id: prev_state.public_id.clone(),
		color: prev_state.color.clone(),
		..default_state()
	}
}

async fn register_client(private_id: String, public_id: &String, selections: UserSelections, clients: Clients) {
	println!("inserting new client");
	clients.write().await.insert(
		private_id,
		Client {
			state: Arc::new(RwLock::new(spawn_with_select(&selections, &public_id))),
			sender: None,
		},
	);
	println!("inserted")
}

pub async fn unregister_handler(private_id: String, clients: Clients) -> Result<impl Reply> {
	println!("removing client");
	clients.write().await.remove(&private_id);
	println!("removed");
	Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, private_id: String, clients: Clients, loot: WorldLoot) -> Result<impl Reply> {
	println!("received websocket, ws_handler (reading all)");
	let client = clients.read().await.get(&private_id).cloned();
	println!("(read all ok)");
	match client {
		Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, private_id, clients, loot, c))),
		None => Err(warp::reject::not_found()),
	}
}

pub async fn health_handler() -> Result<impl Reply> {
	Ok(StatusCode::OK)
}
