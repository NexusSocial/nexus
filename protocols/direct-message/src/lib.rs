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

/*
what do we want from this?
We want the ability to have a convo of direct messages between you and another user
Those messages should be timestamped, and say from whom they came from (DID).
You should be able to delete messages you sent, ( those aren't guarnteed to be deleted by the other party, but the other party
has the option to delete them too if they want to accept your deletion )
We want the ability to edit messages ( same thing as deletion, the other party doesn't nesescarily have to accept ur edits but
they should )
You should be able to reply to specific messages, like the discord reply feature
You should be able to query all the message convos you have with people
You should be able to query all the message convos you have with people, sorted by last interaction
Stuff like Display Name etc isn't a part of this protocol, clients can use other protocols like the Friend
protocol and stuff in order to change the did to a recognizable username?
 */
