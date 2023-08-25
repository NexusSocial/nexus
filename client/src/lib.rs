pub mod non_standard;

use common::client_to_server::{ClientDidMsg, Msg, MsgType, Response};
use common::{Did, DidMsg, Endpoint, Homeserver, Payload, Protocol};
use reqwest::Client;
use crate::non_standard::add_user_to_server;

fn msg_to_endpoint(msg: &Msg) -> String {
	let Msg {
		homeserver,
		protocol,
		did_msg,
	} = msg;
	let DidMsg { payload, did } = did_msg;
	String::from("http://")
		+ &homeserver.0
		+ "/client-to-server/"
		+ match payload {
			MsgType::PopQueue => "pop-queue",
			MsgType::Local(_) => "local",
			MsgType::GeneralRemoteRequest(_) => "general-remote-request",
			MsgType::RemoteRequestTo(_, _) => "remote-request-to",
			MsgType::RemoteSendTo(_, _) => "remote-send-to",
		} + "/" + &protocol.0
}

pub async fn read_queue(
	homeserver: Homeserver,
	protocol: Protocol,
	did: Did,
) -> anyhow::Result<Option<DidMsg<Payload>>> {
	let msg = Msg {
		homeserver,
		did_msg: ClientDidMsg {
			payload: MsgType::PopQueue,
			did,
		},
		protocol,
	};
	let endpoint = msg_to_endpoint(&msg);
	let response = Client::new()
		.get(endpoint)
		.body(serde_json::to_vec(&msg.did_msg).unwrap())
		.send()
		.await?
		.error_for_status()?
		.bytes()
		.await?;
	let response = serde_json::from_slice(&response).unwrap();
	Ok(match response {
		Response::Queue(maybe_did_msg) => maybe_did_msg,
		_ => anyhow::bail!("wrong response, should have been queue"),
	})
}
#[test]
fn test() {
	tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(async {
			let did = Did("malek".to_string());
			let homeserver = Homeserver("127.0.0.1:3000".to_string());
			non_standard::add_user_to_server(did.clone(), homeserver.clone()).await.unwrap();

			let e = read_queue(
				homeserver.clone(),
				Protocol("idk".to_string()),
				did.clone(),
			)
			.await
			.unwrap();
			eprintln!("possible msg is: {:#?}", e);
		});
}
// pub async fn local(homeserver: Homeserver, did_msg: DidMsg) -> anyhow::Result<Payload> {
// 	let msg = Msg {
// 		homeserver,
// 		did_msg,
// 		msg_type: MsgType::Local,
// 	};
// 	let endpoint = msg_to_endpoint(&msg);
// 	let response = Client::new()
// 		.get(endpoint.0)
// 		.json(&msg.did_msg.payload.0)
// 		.send()
// 		.await
// 		.unwrap()
// 		.json::<Response>()
// 		.await
// 		.unwrap();
// 	Ok(match response {
// 		Response::Local(payload) => payload,
// 		_ => anyhow::bail!("wrong response, should have been local"),
// 	})
// }
// pub async fn remote_request(
// 	homeserver: Homeserver,
// 	did_msg: DidMsg,
// 	to: Did,
// ) -> anyhow::Result<DidMsg> {
// 	let msg = Msg {
// 		homeserver,
// 		did_msg,
// 		msg_type: MsgType::RemoteRequest { to },
// 	};
// 	let endpoint = msg_to_endpoint(&msg);
// 	let response: Response = Client::new()
// 		.get(endpoint.0)
// 		.json(&serde_json::to_string(&msg.did_msg.payload).unwrap())
// 		.send()
// 		.await?
// 		.json::<Response>()
// 		.await?;
// 	Ok(match response {
// 		Response::RemoteImmediate(did_msg) => did_msg,
// 		_ => anyhow::bail!("wrong response, should have been remote immediate"),
// 	})
// }
// pub async fn remote_send(
// 	homeserver: Homeserver,
// 	did_msg: DidMsg,
// 	to: Did,
// ) -> anyhow::Result<()> {
// 	let msg = Msg {
// 		homeserver,
// 		did_msg,
// 		msg_type: MsgType::RemoteSend { to },
// 	};
// 	let endpoint = msg_to_endpoint(&msg);
// 	Client::new()
// 		.get(endpoint.0)
// 		.json(&msg.did_msg)
// 		.send()
// 		.await?;
// 	Ok(())
// }
