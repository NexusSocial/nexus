use axum::extract::Path;
use axum::{response::IntoResponse, routing::get, Extension, Json, Router};
use common::client_to_server::Response;
use common::{Did, DidMsg, Homeserver, Payload};
use direct_message::{DirectMessage, LocalMessage};
use reqwest::Client;
use serde::ser;
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub type State = Arc<Mutex<InnerState>>;
#[derive(Default)]
pub struct InnerState {
	users: HashMap<Did, UserState>,
	did_to_homeserver: HashMap<Did, Homeserver>,
}

#[derive(Default)]
pub struct UserState {
	queue: Vec<DidMsg>,
	all_rec_dms: Vec<DirectMessage>,
}

#[tokio::main]
async fn main() {
	// initialize tracing
	let state = Arc::new(Mutex::new(InnerState::default()));

	state.lock().unwrap().did_to_homeserver.insert(
		Did("did-key-malek".to_string()),
		Homeserver("127.0.0.1:3000".to_string()),
	);
	state
		.lock()
		.unwrap()
		.users
		.insert(Did("did-key-malek".to_string()), UserState::default());
	state.lock().unwrap().did_to_homeserver.insert(
		Did("did-key-lyuma".to_string()),
		Homeserver("127.0.0.1:3000".to_string()),
	);
	state
		.lock()
		.unwrap()
		.users
		.insert(Did("did-key-lyuma".to_string()), UserState::default());

	// build our application with a route
	let app = Router::new()
		// `GET /` goes to `root`
		.route("/", get(root))
		.route(
			"/client-to-server/:did/direct-message/:msg-type",
			get(client_to_server_direct_message),
		)
		.route(
			"/server-to-server/:did/direct-message/:msg-type",
			get(server_to_server_direct_message),
		)
		.layer(Extension(state));

	// run our app with hyper
	// `axum::Server` is a re-export of `hyper::Server`
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	println!("listening on {}", addr);
	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
	"Hello, World!"
}

async fn client_to_server_direct_message(
	Extension(state): Extension<Arc<Mutex<InnerState>>>,
	Path((did, msg_type)): Path<(String, String)>,
	Json(payload): Json<Value>,
) -> impl IntoResponse {
	let msg_type =
		common::client_to_server::MsgType::from_url_ending(msg_type).unwrap();
	let did: Did = Did(did);
	match msg_type {
		common::client_to_server::MsgType::Local => {
			let payload = &serde_json::from_value::<Payload>(payload).unwrap().0;
			println!("payload is: {}", payload);
			let mut payload = payload.clone();
			let r#type: direct_message::LocalMessage =
				serde_json::from_str(&payload).unwrap();
			match r#type {
				LocalMessage::AddDm(dm) => {
					state
						.lock()
						.unwrap()
						.users
						.get_mut(&did)
						.unwrap()
						.all_rec_dms
						.push(dm);
					serde_json::to_string(&Response::Local(Payload("".to_string())))
						.unwrap()
				}
				LocalMessage::ReadAllMsgs => serde_json::to_string(
					&common::client_to_server::Response::Local(Payload(
						serde_json::to_string(
							&state
								.lock()
								.unwrap()
								.users
								.get(&did)
								.unwrap()
								.all_rec_dms
								.iter()
								.map(|a| a.clone())
								.collect::<Vec<_>>(),
						)
						.unwrap(),
					)),
				)
				.unwrap(),
			}
		}
		common::client_to_server::MsgType::PopQueue => {
			let msg = common::client_to_server::Response::Queue(
				state
					.lock()
					.unwrap()
					.users
					.get_mut(&did)
					.unwrap_or_else(|| panic!("incorrect did: {}", did))
					.queue
					.pop(),
			);
			let msg = serde_json::to_string(&msg).unwrap();
			println!("responding with: {}", msg);
			msg
		}
		common::client_to_server::MsgType::RemoteRequest { .. } => {
			unimplemented!()
		}
		common::client_to_server::MsgType::RemoteSend { to } => {
			println!("in remote send");
			let homeserver = state
				.lock()
				.unwrap()
				.did_to_homeserver
				.get(&to)
				.unwrap()
				.clone();
			let endpoint = format!(
				"http://{}/server-to-server/{}/direct-message/{}",
				homeserver,
				to,
				serde_json::to_string(&common::server_to_server::MsgType::RemoteSend)
					.unwrap()
			);
			let did_msg: DidMsg = serde_json::from_value(payload).unwrap();
			println!(
				"{}",
				Client::new()
					.get(endpoint)
					.json(&did_msg)
					.send()
					.await
					.unwrap()
					.text()
					.await
					.unwrap()
			);
			"".to_string()
		}
	}
}

async fn server_to_server_direct_message(
	Extension(state): Extension<Arc<Mutex<InnerState>>>,
	Path((did, msg_type)): Path<(String, String)>,
	Json(payload): Json<Value>,
) -> impl IntoResponse {
	let msg_type: common::server_to_server::MsgType =
		serde_json::from_str(&msg_type).unwrap();
	let did: Did = Did(did);
	match msg_type {
		common::server_to_server::MsgType::RemoteRequest => unimplemented!(),
		common::server_to_server::MsgType::RemoteSend => {
			println!("pushing did msg of: {}", payload);
			let did_msg: DidMsg = serde_json::from_value(payload).unwrap();
			state
				.lock()
				.unwrap()
				.users
				.get_mut(&did)
				.unwrap()
				.queue
				.push(did_msg);
		}
	}

	"".to_string()
}
