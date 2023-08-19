mod asymmetric;
mod did;

/// The result of encryption. Newtype for type safety.
#[derive(Debug, Eq, PartialEq)]
pub struct CipherText(Vec<u8>);
