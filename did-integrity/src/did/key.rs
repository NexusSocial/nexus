use std::str::FromStr;

use crate::did::DidDocument;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
	#[error("must be did: schema")]
	WrongSchema,
	#[error("must be did:key method")]
	WrongMethod,
	#[error("did:key: must be followed by a multibase encoded string")]
	NotMultibase(#[from] multibase::Error),
	#[error("the multibase encoding must be Base58Btc")]
	NotBase58Btc,
}
/// Must be a valid `did:key`.
// TODO: Still need to do multicodec parsing
#[derive(Debug, Eq, PartialEq)]
pub struct DidKey(Vec<u8>);

impl FromStr for DidKey {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let encoded = s
			.strip_prefix("did:")
			.ok_or(ParseError::WrongSchema)?
			.strip_prefix("key:")
			.ok_or(ParseError::WrongMethod)?;
		let (base, decoded) = multibase::decode(encoded)?;
		if base != multibase::Base::Base58Btc {
			return Err(ParseError::NotBase58Btc);
		}
		Ok(Self(decoded))
	}
}

impl DidKey {
	/// Resolves the key to its document. This is possible without any IO, as a
	/// `did:key` is self-describing.
	pub fn resolve() -> DidDocument {
		todo!()
	}
}

#[cfg(test)]
mod tests {}
