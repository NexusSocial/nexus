use axum::body::HttpBody;
use axum::routing::get;
use axum::{Extension, Router};
use common::{Did, DidMsg, Homeserver, Payload, Protocol};
use serde::ser::StdError;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Default, Clone)]
pub struct Clients(pub Arc<Mutex<HashMap<Did, ClientData>>>);
#[derive(Default, Clone)]
pub struct HomeserverMappings(pub Arc<Mutex<HashMap<Did, Homeserver>>>);

#[derive(Default)]
pub struct ClientData {
	msg_to_read: HashMap<Protocol, Vec<DidMsg<Payload>>>,
}
pub fn add_routes<S, B>(router: Router<S, B>) -> Router<S, B>
where
	B: HttpBody + 'static,
	B: Send,
	S: Clone + 'static,
	S: Send,
	S: Sync,
	<B as HttpBody>::Data: Send,
	<B as HttpBody>::Error: StdError,
	<B as HttpBody>::Error: Send,
	<B as HttpBody>::Error: Sync,
{
	router
		.route(
			"/client-to-server/pop-queue/:protocol",
			get(client_to_server::pop_queue),
		)
		.route(
			"/client-to-server/remote-send-to/:protocol",
			get(client_to_server::remote_send_to),
		)
		.route(
			"/server-to-server/remote-send-to/:protocol",
			get(server_to_server::remote_send_to),
		)
		.layer(Extension(Clients::default()))
		.layer(Extension(HomeserverMappings::default()))
}

mod client_to_server {
	use crate::vanilla::{Clients, HomeserverMappings};
	use axum::body::Bytes;
	use axum::extract::Path;
	use axum::http::StatusCode;
	use axum::response::IntoResponse;
	use axum::Extension;
	use axum_macros::debug_handler;
	use common::client_to_server::*;
	use common::{DidMsg, Payload, Protocol};

	#[debug_handler]
	pub async fn pop_queue(
		Extension(clients): Extension<Clients>,
		Path(protocol): Path<String>,
		body: Bytes,
	) -> Result<impl IntoResponse, StatusCode> {
		Ok(serde_json::to_vec(&Response::Queue(
			clients
				.0
				.lock()
				.map_err(|err| { eprintln!("{}", err); StatusCode::INTERNAL_SERVER_ERROR })?
				.get_mut(
					&serde_json::from_slice::<ClientDidMsg>(&body)
						.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap()
						.did,
				)
				.ok_or(StatusCode::INTERNAL_SERVER_ERROR).unwrap()
				.msg_to_read
				.entry(Protocol(protocol))
				.or_default()
				.pop())
		)
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap())
	}
	pub async fn remote_send_to(
		Extension(homeserver_mappings): Extension<HomeserverMappings>,
		body: Bytes,
	) -> impl IntoResponse {
		let msg: Msg = serde_json::from_slice(&body).unwrap();
		let did_msg = msg.did_msg;
		let protocol = msg.protocol.0;
		match did_msg.payload {
			MsgType::RemoteSendTo(did_to, payload_msg) => {
				let msg = common::server_to_server::Msg {
					to_homeserver: homeserver_mappings
						.0
						.lock()
						.unwrap()
						.get(&did_to)
						.unwrap()
						.clone(),
					to_did: did_to,
					did_msg: payload_msg,
				};
				reqwest::Client::new()
					.get(
						msg.to_homeserver.clone().0
							+ "/server-to-server/remote-send-to/"
							+ &protocol,
					)
					.body(serde_json::to_vec(&msg).unwrap())
					.send()
					.await
					.unwrap()
					.bytes()
					.await
					.unwrap()
			}
			_ => panic!(),
		}
	}
}
mod server_to_server {
	use crate::vanilla::Clients;
	use axum::body::Bytes;
	use axum::extract::Path;
	use axum::response::IntoResponse;
	use axum::Extension;
	use common::server_to_server::Msg;
	use common::Protocol;

	pub async fn remote_send_to(
		Extension(clients): Extension<Clients>,
		Path(protocol): Path<String>,
		body: Bytes,
	) -> impl IntoResponse {
		let server_msg: Msg = serde_json::from_slice(&body).unwrap();
		let protocol = Protocol(protocol);
		match server_msg {
			Msg {
				to_did, did_msg, ..
			} => {
				if !clients
					.0
					.lock()
					.unwrap()
					.get(&to_did)
					.unwrap()
					.msg_to_read
					.contains_key(&protocol)
				{
					clients
						.0
						.lock()
						.unwrap()
						.get_mut(&to_did)
						.unwrap()
						.msg_to_read
						.insert(protocol.clone(), vec![]);
				}
				clients
					.0
					.lock()
					.unwrap()
					.get_mut(&to_did)
					.unwrap()
					.msg_to_read
					.get_mut(&protocol)
					.unwrap()
					.push(did_msg);
				serde_json::to_string(
					&common::server_to_server::Response::MsgQueuedSuccessful,
				)
				.unwrap()
			}
		}
	}
}
