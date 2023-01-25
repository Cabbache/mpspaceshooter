use crate::{ws, Client, Clients, Result};
use serde::{Serialize};
use serde_json::Value;
use uuid::Uuid;
use warp::{http::StatusCode, reply::json, Reply};
use std::fs;
use std::time::Instant;

use crate::PlayerState;
use crate::game::Color;
use crate::game::MotionStart;
use crate::game::PlayerMotion;

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
	//url: String,
	public: String,
	private: String,
}

pub async fn register_handler(_body: Value, clients: Clients) -> Result<impl Reply> {
	let private_uuid = Uuid::new_v4().as_simple().to_string();
	let public_uuid = Uuid::new_v4().as_simple().to_string();

	println!("got reg");
	register_client(
		private_uuid.clone(),
		public_uuid.clone(),
		clients,
	).await;
	Ok(json(&RegisterResponse {
		//url: format!("ws://127.0.0.1:8000/ws/{}", uuid),
		public: public_uuid,
		private: private_uuid,
	}))
}

pub async fn serve_page() -> Result<impl Reply> {
	let html = fs::read_to_string("site.html").unwrap();
	Ok(warp::reply::html(html))
}

async fn register_client(private_id: String, public_id: String, clients: Clients) {
	clients.write().await.insert(
		private_id,
		Client {
			state: PlayerState{
				name: "Bob".to_string(),
				public_id: public_id,
				x:0,
				y:0,
				color: Color{r:0,g:0,b:0},
				motion: MotionStart{direction: PlayerMotion::Stopped, time: Instant::now()}
			},
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
