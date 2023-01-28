use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};

mod handler;
mod game;
mod ws;

use crate::game::PlayerState;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
	pub state: PlayerState,
	pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
async fn main() {
	let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

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
		.and_then(handler::ws_handler);

	let routes = health_route
		.or(register_routes)
		.or(ws_route)
		.or(index)
		.or(assets)
		.with(warp::cors().allow_any_origin());

	warp::serve(routes)
	//.tls()
	//.cert_path("cert.pem")
	//.key_path("key.rsa")
	.run(([0, 0, 0, 0], 80)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
	warp::any().map(move || clients.clone())
}
