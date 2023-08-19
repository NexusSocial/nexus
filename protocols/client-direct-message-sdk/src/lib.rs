use anyhow::Result;
use client::read_queue;
use common::{Did, DidMsg, Homeserver, Payload, Protocol};
use direct_message::{DirectMessage, LocalMessage};

pub struct Client {
	pub homeserver: Homeserver,
	pub did: Did,
	pub username: String,
}
impl Client {
	pub fn protocol() -> Protocol {
		Protocol(String::from("direct-message"))
	}
	pub fn new(homeserver: Homeserver, did: Did, username: String) -> Self {
		Self {
			homeserver,
			did,
			username,
		}
	}
	pub async fn send_dm(&self, dm: DirectMessage, to: Did) -> Result<()> {
		client::remote_send(
			self.homeserver.clone(),
			DidMsg {
				payload: Payload(serde_json::to_string(&dm).unwrap()),
				protocol: Self::protocol(),
				did: self.did.clone(),
			},
			to,
		)
		.await?;
		Ok(())
	}
	pub async fn read_dms(&self) -> Result<Vec<DirectMessage>> {
		// first we read all the message queues we have and store them back in the server.
		while let Some(did_msg) =
			read_queue(self.homeserver.clone(), Self::protocol(), self.did.clone())
				.await
				.unwrap()
		{
			client::local(
				self.homeserver.clone(),
				DidMsg {
					payload: Payload(
						serde_json::to_string(&direct_message::LocalMessage::AddDm(
							serde_json::from_str(&did_msg.payload.0).unwrap(),
						))
						.unwrap(),
					),
					protocol: Self::protocol(),
					did: self.did.clone(),
				},
			)
			.await
			.unwrap();
		}
		//now we read all our stored dms
		let payload = client::local(
			self.homeserver.clone(),
			DidMsg {
				payload: Payload(
					serde_json::to_string(&direct_message::LocalMessage::ReadAllMsgs)
						.unwrap(),
				),
				protocol: Self::protocol(),
				did: self.did.clone(),
			},
		)
		.await
		.unwrap();
		let msgs = serde_json::from_str(&payload.0).unwrap();
		Ok(msgs)
	}
}
