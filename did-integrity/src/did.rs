//! Module for everything specific to Decentralized Identifiers (DIDs).

use crate::asymmetric::{PublicKey, Signature};

/// A decentralized identifier.
#[derive(Debug, Eq, PartialEq)]
pub enum Did {
	Key(DidKey),
	Web(DidWeb),
}

/// Everything after `did:key:`.
#[derive(Debug, Eq, PartialEq)]
pub struct DidKey(String);
impl DidKey {
	/// Resolves the key to its document. This is possible without any IO, as a
	/// `did:key` is self-describing.
	fn resolve() -> DidDocument {
		todo!()
	}
}

/// Everything after `did:web:`.
#[derive(Debug, Eq, PartialEq)]
pub struct DidWeb(String);

/// A [`Did`] resolves to a [`DidDocument`].
#[derive(Debug, Eq, PartialEq)]
struct DidDocument {
	/// A `DidDocument` is self describing in that it contains the `Did` too.
	pub did: Did,
	pub pubkey: PublicKey,
}

/// A chain of [`Did`]s which start at a [`RootDid`] that signs a child, which
/// signs its child, and so on.
#[derive(Debug, Eq, PartialEq)]
pub struct DidChain {
	root: RootDid,
	chain: Vec<Did>,
	signatures: Vec<Signature>,
}
impl DidChain {
	pub fn root(&self) -> &RootDid {
		&self.root
	}

	/// NOTE: This may be the same as `root` if there are no child `Did`s.
	pub fn leaf(&self) -> &Did {
		self.chain.last().unwrap_or(&self.root.0)
	}

	/// The number of `Did`s in the chain, including the root.
	#[allow(clippy::len_without_is_empty)]
	pub fn len(&self) -> u8 {
		assert!(self.chain.len() < u8::MAX as usize);
		assert_eq!(self.chain.len(), self.signatures.len());
		1 + u8::try_from(self.chain.len()).unwrap()
	}
}

/// Newtype on [`Did`] that indicates that this is a root.
#[derive(Debug, Eq, PartialEq)]
pub struct RootDid(Did);
