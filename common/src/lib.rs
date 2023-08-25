pub mod non_standard;

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
/// uselss doc
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct DidMsg<GenericPayload> {
	pub payload: GenericPayload,
	pub did: Did,
}

// #[derive(
// 	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
// )]
// pub struct Payload(pub String);
pub type Payload = Vec<u8>;

#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct Did(pub String);
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct Protocol(pub String);
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct Homeserver(pub String);
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct Endpoint(pub String);
impl Endpoint {
	pub fn new(endpoint: String) -> Self {
		Self(endpoint)
	}
}

impl Display for Homeserver {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl Display for Protocol {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}
impl Display for Did {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

pub mod client_to_server {
	use crate::{Did, DidMsg, Homeserver, Payload, Protocol};
	use serde::{Deserialize, Serialize};

	pub type ClientDidMsg = DidMsg<MsgType>;

	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum Response {
		Local(Payload),
		Queue(Option<DidMsg<Payload>>),
		RemoteImmediate(ClientDidMsg),
	}
	impl Default for Response {
		fn default() -> Self {
			Self::Local(Payload::default())
		}
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub struct Msg {
		pub homeserver: Homeserver,
		pub protocol: Protocol,
		pub did_msg: ClientDidMsg,
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum MsgType {
		PopQueue,
		Local(Payload),
		GeneralRemoteRequest(Payload),
		RemoteRequestTo(Did, Payload),
		RemoteSendTo(Did, DidMsg<Payload>),
	}
}
pub mod server_to_server {
	use crate::*;
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum Response {
		RemoteImmediate(DidMsg<Payload>),
		MsgQueuedSuccessful,
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub struct Msg {
		pub to_homeserver: Homeserver,
		pub to_did: Did,
		pub did_msg: DidMsg<Payload>,
	}
}
