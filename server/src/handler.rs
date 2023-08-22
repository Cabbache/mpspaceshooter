use crate::{ws, Client, Clients, Result};
use serde::{Serialize, Deserialize};
use warp::{http::StatusCode, reply::json, Reply};
use xxhash_rust::xxh3::xxh3_64;
use uuid::Uuid;
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::fs;

use crate::PlayerState;
use crate::WorldLoot;
use utils::server_gameobjects::*;
use utils::trajectory::*;

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
				return Ok(json(&"meow")) //TODO
			}

			let private_uuid = Uuid::new_v4().as_simple().to_string();
			let public_id = format!("{:x}", xxh3_64(private_uuid.as_bytes()));
			println!("Registering client {}", private_uuid);
			register_client(
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

fn default_state() -> PlayerState {
	PlayerState {
		name: "".to_string(),
		id: "".to_string(),
		health: 100.0,
		cash: 20,
		fuel: 100,
		color: Color{r:255,g:255,b:255},
		trigger_pressed: false,
		inventory: Inventory{
			selection: 0,
			weapons: HashMap::from([
				(0, Weapon{ weptype: WeaponType::Pistol, ammo: 50 }),
				(1, Weapon{ weptype: WeaponType::Grenade {press_time: current_time() as f32}, ammo: 2 }),
			])
		},
		trajectory: Trajectory::default()
	}
}

pub fn spawn_with_select(selections: &UserSelections, public_id: &String) -> PlayerState {
	PlayerState{
		name: selections.nick.clone(),
		id: public_id.clone(),
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
		id: prev_state.id.clone(),
		color: prev_state.color.clone(),
		..default_state()
	}
}

async fn register_client(public_id: String, selections: UserSelections, clients: Clients) {
	clients.write().await.insert(
		public_id.clone(),
		Client {
			state: Arc::new(RwLock::new(spawn_with_select(&selections, &public_id))),
			sender: None,
		},
	);
	println!("inserted {}", public_id);
}

pub async fn unregister_handler(public_id: String, clients: Clients) -> Result<impl Reply> {
	println!("removing client");
	clients.write().await.remove(&public_id);
	println!("removed");
	Ok(StatusCode::OK)
}

pub async fn ws_handler(ws: warp::ws::Ws, private_id: String, clients: Clients, loot: WorldLoot) -> Result<impl Reply> {
	let public_id = format!("{:x}", xxh3_64(private_id.as_bytes()));
	println!("Received connection from {}", public_id);
	let client = clients.read().await.get(&public_id).cloned();
	match client {
		Some(c) => Ok(ws.on_upgrade(move |socket| ws::client_connection(socket, public_id, clients, loot, c))),
		None => Err(warp::reject::not_found()),
	}
}

pub async fn health_handler() -> Result<impl Reply> {
	Ok(StatusCode::OK)
}
