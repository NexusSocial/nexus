mod vanilla;

use crate::vanilla::{ClientData, Clients, HomeserverMappings};
use axum::body::Bytes;
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use std::net::SocketAddr;
use reqwest::StatusCode;

#[tokio::main]
async fn main() {
	let app = Router::new().route("/add-user", get(add_user)).route(
		"/add-user-homeserver-mapping",
		get(add_user_homeserver_mapping),
	);
	let app = vanilla::add_routes(app);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	println!("listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn add_user(Extension(clients): Extension<Clients>, body: Bytes) -> impl IntoResponse {
	let add_user =
		serde_json::from_slice::<common::non_standard::AddUser>(&body).unwrap();
	clients
		.0
		.lock()
		.unwrap()
		.insert(add_user.did, ClientData::default());
	StatusCode::ACCEPTED
}
async fn add_user_homeserver_mapping(
	Extension(homeserver_mappings): Extension<HomeserverMappings>,
	body: Bytes,
) -> impl IntoResponse {
	let homeserver_mapping =
		serde_json::from_slice::<common::non_standard::AddHomeServerMapping>(&body)
			.unwrap();
	homeserver_mappings
		.0
		.lock()
		.unwrap()
		.insert(homeserver_mapping.did, homeserver_mapping.homeserver);
	StatusCode::ACCEPTED
}

// basic handler that responds with a static string
async fn root() -> &'static str {
	"Hello, World!"
}
