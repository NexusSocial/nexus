use common::client_to_server::{Msg, MsgType, Response};
use common::{Did, DidMsg, Endpoint, Homeserver, Payload, Protocol};
use reqwest::Client;

fn msg_to_endpoint(
	Msg {
		homeserver,
		did_msg,
		msg_type,
	}: &Msg,
) -> Endpoint {
	let DidMsg { protocol, did, .. } = did_msg;
	Endpoint::new(format!(
		"http://{}/client-to-server/{}/{}/{}",
		homeserver,
		did,
		protocol,
		msg_type.to_url_ending()
	))
}
pub async fn read_queue(
	homeserver: Homeserver,
	protocol: Protocol,
	did: Did,
) -> anyhow::Result<Option<DidMsg>> {
	let msg = Msg {
		homeserver,
		did_msg: DidMsg {
			payload: Payload(String::from("")),
			protocol,
			did,
		},
		msg_type: MsgType::PopQueue,
	};
	let endpoint = msg_to_endpoint(&msg);
	let response = Client::new()
		.get(endpoint.0)
		.json(&serde_json::to_string(&msg.did_msg.payload).unwrap())
		.send()
		.await?
		.json::<Response>()
		.await?;
	Ok(match response {
		Response::Queue(maybe_did_msg) => maybe_did_msg,
		_ => anyhow::bail!("wrong response, should have been queue"),
	})
}
pub async fn local(homeserver: Homeserver, did_msg: DidMsg) -> anyhow::Result<Payload> {
	let msg = Msg {
		homeserver,
		did_msg,
		msg_type: MsgType::Local,
	};
	let endpoint = msg_to_endpoint(&msg);
	let response = Client::new()
		.get(endpoint.0)
		.json(&msg.did_msg.payload.0)
		.send()
		.await
		.unwrap()
		.json::<Response>()
		.await
		.unwrap();
	Ok(match response {
		Response::Local(payload) => payload,
		_ => anyhow::bail!("wrong response, should have been local"),
	})
}
pub async fn remote_request(
	homeserver: Homeserver,
	did_msg: DidMsg,
	to: Did,
) -> anyhow::Result<DidMsg> {
	let msg = Msg {
		homeserver,
		did_msg,
		msg_type: MsgType::RemoteRequest { to },
	};
	let endpoint = msg_to_endpoint(&msg);
	let response: Response = Client::new()
		.get(endpoint.0)
		.json(&serde_json::to_string(&msg.did_msg.payload).unwrap())
		.send()
		.await?
		.json::<Response>()
		.await?;
	Ok(match response {
		Response::RemoteImmediate(did_msg) => did_msg,
		_ => anyhow::bail!("wrong response, should have been remote immediate"),
	})
}
pub async fn remote_send(
	homeserver: Homeserver,
	did_msg: DidMsg,
	to: Did,
) -> anyhow::Result<()> {
	let msg = Msg {
		homeserver,
		did_msg,
		msg_type: MsgType::RemoteSend { to },
	};
	let endpoint = msg_to_endpoint(&msg);
	Client::new()
		.get(endpoint.0)
		.json(&msg.did_msg)
		.send()
		.await?;
	Ok(())
}
