use serde::{Deserialize, Serialize};

#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct DirectMessage {
	pub from: String,
	pub contents: String,
}
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub enum LocalMessage {
	AddDm(DirectMessage),
	ReadAllMsgs,
}
