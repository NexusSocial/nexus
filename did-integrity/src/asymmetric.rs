//! Module for standard asymmetric cryptography

use crate::CipherText;

// TODO: Figure out how to represent this
#[derive(Debug, Eq, PartialEq)]
pub struct PublicKey(String);
impl PublicKey {
	fn encrypt(&self, _plaintext: &[u8]) -> CipherText {
		todo!()
	}
}
impl signature::Verifier<Signature> for PublicKey {
	fn verify(
		&self,
		_msg: &[u8],
		_signature: &Signature,
	) -> Result<(), signature::Error> {
		todo!()
	}
}

// TODO: How to represent this?
pub struct SecretKey(String);
impl core::fmt::Debug for SecretKey {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("SecretKey").finish_non_exhaustive()
	}
}
impl signature::Signer<Signature> for SecretKey {
	fn try_sign(&self, _msg: &[u8]) -> Result<Signature, signature::Error> {
		todo!()
	}
}

// TODO: Figure out how to represent this.
#[derive(Debug, Eq, PartialEq)]
pub struct Signature(String);
