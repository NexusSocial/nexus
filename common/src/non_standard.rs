use crate::{Did, Homeserver};
use serde::{Deserialize, Serialize};
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct AddUser {
	pub did: Did,
}
#[derive(
	Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Debug, Serialize, Deserialize,
)]
pub struct AddHomeServerMapping {
	pub did: Did,
	pub homeserver: Homeserver,
}
