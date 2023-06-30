use base58::{FromBase58, ToBase58};
use sha2::{Digest, Sha256};

use crate::{core::Runtime, plugins::hasher::Hasher};

/// TODO: extract this function in the core module
/// And find a way to have a generic way to generate to_b58 and from_b58
fn from_base58(prefix_size: usize, encoded: &str) -> Result<Vec<u8>, ()> {
    let check_sum_size = 4;
    let decoded = encoded.from_base58().map_err(|_| ())?;
    let _payload = &decoded[..decoded.len() - check_sum_size]; // Remove the check sum
    let bytes = decoded[prefix_size..].to_vec();
    let bytes = bytes[..bytes.len() - check_sum_size].to_vec();
    Ok(bytes)
}

/// TODO: extract this
fn to_base58(prefix: Vec<u8>, data: &[u8]) -> String {
    let mut result = prefix;
    result.extend(data);
    let checksum = Sha256::digest(&Sha256::digest(&result)).to_vec()[..4].to_vec();
    result.extend(checksum);
    result.to_base58()
}

#[derive(Clone)]
pub enum PublicKey {
    /// tz1 address
    Ed25519(ed25519_compact::PublicKey),
}

#[derive(Clone)]
pub enum Signature {
    Ed25519(ed25519_compact::Signature),
}

impl TryFrom<String> for PublicKey {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let public_key = from_base58(4, &value)?;
        let ed25519 = ed25519_compact::PublicKey::from_slice(&public_key).map_err(|_| ())?;
        Ok(PublicKey::Ed25519(ed25519))
    }
}

impl ToString for PublicKey {
    fn to_string(&self) -> String {
        match self {
            PublicKey::Ed25519(ed25519) => {
                let bytes = ed25519.as_ref();

                to_base58(vec![13, 15, 37, 217], bytes)
            }
        }
    }
}

impl TryFrom<String> for Signature {
    type Error = ();
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let signature = from_base58(5, &value)?;
        let sig = ed25519_compact::Signature::from_slice(&signature).map_err(|_| ())?;
        Ok(Signature::Ed25519(sig))
    }
}

pub trait Verifier {
    /// Verify the signature for a given public key over the hash of the given data
    ///
    /// The data passed as parameter will be hashed
    /// Then the public will be used to verify that the signature is indeed the signature of the generated hash
    fn verify_signature(
        &mut self,
        signature: &Signature,
        public_key: &PublicKey,
        data: &[u8],
    ) -> bool;
}

impl<R> Verifier for R
where
    R: Runtime + Hasher,
{
    fn verify_signature(
        &mut self,
        signature: &Signature,
        public_key: &PublicKey,
        data: &[u8],
    ) -> bool {
        let data = self.hash(data);

        match (signature, public_key) {
            (Signature::Ed25519(sig), PublicKey::Ed25519(pkey)) => {
                let res = pkey.verify(data, sig);
                res.is_ok()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicKey, Signature, Verifier};
    use crate::core::MockRuntime;

    #[test]
    fn test_ed25519_pkey_deserialization() {
        let string = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK".to_string();
        let pkey = PublicKey::try_from(string);

        assert!(pkey.is_ok())
    }

    #[test]
    fn test_ed25519_pkey_serialization() {
        let string = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK".to_string();
        let pkey = PublicKey::try_from(string.clone()).unwrap().to_string();

        assert_eq!(string, pkey);
    }

    #[test]
    fn test_ed25519_signature_deserialization() {
        let encoded = "edsigtuU5nUqBniorqTFXFixkG6ZkfvEPrfc9aT9DnMAeims2AX2yjpgYaedXBoKzAGHE3ZXSi1hZz6piZ3itTE7f2F4FoaxXtM";
        let signature = Signature::try_from(encoded.to_string());

        assert!(signature.is_ok())
    }

    #[test]
    fn test_verify_ed25519_signature() {
        let mut rt = MockRuntime::default();

        let encoded = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK";
        let public_key = PublicKey::try_from(encoded.to_string()).unwrap();

        /////////
        let encoded = "edsigtuU5nUqBniorqTFXFixkG6ZkfvEPrfc9aT9DnMAeims2AX2yjpgYaedXBoKzAGHE3ZXSi1hZz6piZ3itTE7f2F4FoaxXtM";
        let signature = Signature::try_from(encoded.to_string()).unwrap();

        //// Check the signature
        let data = "hello world".as_bytes();

        let is_ok = rt.verify_signature(&signature, &public_key, data);

        assert!(is_ok);
    }
}
