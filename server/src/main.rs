use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use std::env;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

//use utils::server_gameobjects::LootContent;
//use uuid::Uuid;

mod handler;
mod game;
mod ws;

use utils::server_gameobjects::{PlayerState, LootObject};

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type WorldLoot = Arc<RwLock<HashMap<String, LootObject>>>;

#[derive(Debug, Clone)]
pub struct Client {
	pub state: Arc<RwLock<PlayerState>>,
	pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
	let clients: Clients = Arc::new(RwLock::new(HashMap::new()));
	let world_loot: WorldLoot = Arc::new(RwLock::new(HashMap::new()));
//	{
//		let mut loot_writer = world_loot.write().await;
//		for x in -150..150 {
//			loot_writer.insert(
//				Uuid::new_v4().as_simple().to_string(),
//				LootObject {
//					x: (x as f32)*100f32,
//					y: (x as f32)*100f32,
//					loot: LootContent::Cash(1),
//				}
//			);
//		}
//	}

	let health_route = warp::path!("health").and_then(handler::health_handler);

	let register = warp::path("register");
	let register_routes = register
		.and(warp::post())
		.and(warp::body::json())
		.and(with_clients(clients.clone()))
		.and_then(handler::register_handler)
		.or(register
			.and(warp::delete())
			.and(warp::path::param())
			.and(with_clients(clients.clone()))
			.and_then(handler::unregister_handler));

	let index = warp::path::end()
		.and(warp::get())
		.and_then(handler::serve_page);
	
	let assets = warp::path("static")
		.and(warp::fs::dir("client"));

	let ws_route = warp::path("ws")
		.and(warp::ws())
		.and(warp::path::param())
		.and(with_clients(clients.clone()))
		.and(with_loot(world_loot.clone()))
		.and_then(handler::ws_handler);

	let routes = health_route
		.or(register_routes)
		.or(ws_route)
		.or(index)
		.or(assets)
		.with(warp::cors().allow_any_origin());

	const USAGE: &str = "Usage: ./binary <port>";
	let port: u16 = env::args().nth(1)
		.expect(USAGE)
		.parse()
		.expect(USAGE);

	warp::serve(routes)
	//.tls()
	//.cert_path("cert.pem")
	//.key_path("key.rsa")
	.run(([0, 0, 0, 0], port)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
	warp::any().map(move || clients.clone())
}

fn with_loot(loot: WorldLoot) -> impl Filter<Extract = (WorldLoot,), Error = Infallible> + Clone {
	warp::any().map(move || loot.clone())
}
