use crate::{ws, Client, Clients, Result};
use serde::{Serialize, Deserialize};
use warp::{http::StatusCode, reply::json, Reply};
use rand_distr::{Normal, Distribution};
use xxhash_rust::xxh3::xxh3_64;
use uuid::Uuid;
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::fs;

use crate::PlayerState;
use crate::WorldLoot;
use crate::game::Color;
use utils::{Trajectory, Vector};
use crate::game::Inventory;
use crate::game::Weapon;
use crate::game::WeaponType;
use crate::game::current_time;

const SPAWN_PULL_MAX: f32 = 10.0;

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

fn gen_spawn() -> Vector {
	let normal = Normal::new(-500.0, 500.0).unwrap();
	let mut pos: Vector;
	loop {
		pos = Vector{
			x: normal.sample(&mut rand::thread_rng()),
			y: normal.sample(&mut rand::thread_rng()),
		};
		let psum = Trajectory::pull_sum(&pos);
		if psum.x.powf(2.0) + psum.y.powf(2.0) < SPAWN_PULL_MAX.powf(2.0) {
			break;
		}
	}
	pos
}

fn default_state() -> PlayerState {
	return PlayerState {
		name: "".to_string(),
		public_id: "".to_string(),
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
		trajectory: Trajectory{
			propelling: false,
			pos: gen_spawn(),
			vel: Vector{x: 0.0, y: 0.0},
			spin_direction: 0,
			spin: 0.0,
			time: current_time(),
			collision: false,
		}
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
