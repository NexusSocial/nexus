use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
/// uselss doc
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct DidMsg {
	pub payload: Payload,
	pub protocol: Protocol,
	pub did: Did,
}

#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct Payload(pub String);
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
	use crate::*;
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum MsgType {
		Local,
		PopQueue,
		RemoteRequest { to: Did },
		RemoteSend { to: Did },
	}
	impl MsgType {
		pub fn to_url_ending(&self) -> String {
			match &self {
				MsgType::Local => format!("local"),
				MsgType::PopQueue => format!("pop-queue"),
				MsgType::RemoteRequest { to } => format!("remote-request!{}", to),
				MsgType::RemoteSend { to } => format!("remote-send!{}", to),
			}
		}
		pub fn from_url_ending(string: String) -> Option<Self> {
			if string.starts_with("local") {
				return Some(MsgType::Local);
			}
			if string.starts_with("pop-queue") {
				return Some(MsgType::PopQueue);
			}
			if string.starts_with("remote-request") {
				return Some(MsgType::RemoteRequest {
					to: Did(string.replace("remote-request!", "")),
				});
			}
			if string.starts_with("remote-send") {
				return Some(MsgType::RemoteSend {
					to: Did(string.replace("remote-send!", "")),
				});
			}
			None
		}
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum Response {
		Local(Payload),
		Queue(Option<DidMsg>),
		RemoteImmediate(DidMsg),
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub struct Msg {
		pub homeserver: Homeserver,
		pub did_msg: DidMsg,
		pub msg_type: MsgType,
	}
}
pub mod server_to_server {
	use crate::*;
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum Response {
		RemoteImmediate(DidMsg),
		MsgQueued,
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub enum MsgType {
		RemoteRequest,
		RemoteSend,
	}
	impl Display for MsgType {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			match &self {
				MsgType::RemoteRequest => write!(f, "remote-request"),
				MsgType::RemoteSend => write!(f, "remote-send"),
			}
		}
	}
	#[derive(
		Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
	)]
	pub struct Msg {
		pub to_homeserver: Homeserver,
		pub did_msg: DidMsg,
		pub to_did: Did,
		pub msg_type: MsgType,
	}
}
