use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

use crate::core::CustomRuntime;

//use tezos_smart_rollup_host::runtime::Runtime;

#[derive(PartialEq, Eq, Debug)]
pub struct Blake2b512([u8; 64]);

#[derive(PartialEq, Eq, Debug)]
pub struct Blake2b256([u8; 32]);

pub trait Hasher {
    /// Hash any data to a Blake2b 256 bits
    fn hash(&mut self, data: &[u8]) -> Blake2b256;

    /// Hash any data to a Blake2b 512 bits
    fn hash_512(&mut self, data: &[u8]) -> Blake2b512;
}

impl ToString for Blake2b512 {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .fold(String::default(), |acc, u8| format!("{}{:02x}", acc, u8))
    }
}

impl ToString for Blake2b256 {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .fold(String::default(), |acc, u8| format!("{}{:02x}", acc, u8))
    }
}

impl AsRef<[u8]> for Blake2b512 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Blake2b256 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<R> Hasher for R
where
    R: CustomRuntime,
{
    fn hash(&mut self, data: &[u8]) -> Blake2b256 {
        let mut hasher = Blake2bVar::new(32).unwrap();
        hasher.update(data.as_ref());
        let mut buf = [0u8; 32];
        hasher.finalize_variable(&mut buf).unwrap();
        Blake2b256(buf)
    }

    fn hash_512(&mut self, data: &[u8]) -> Blake2b512 {
        let mut hasher = Blake2bVar::new(64).unwrap();
        hasher.update(data.as_ref());
        let mut buf = [0u8; 64];
        hasher.finalize_variable(&mut buf).unwrap();
        Blake2b512(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::Hasher;
    use crate::core::MockRuntime;

    #[test]
    fn test_hash_512() {
        let mut runtime = MockRuntime::default();
        let data = "hello world".as_bytes().to_vec();
        let hash = runtime.hash_512(&data);
        let hash = hash.to_string();
        assert_eq!(hash, "021ced8799296ceca557832ab941a50b4a11f83478cf141f51f933f653ab9fbcc05a037cddbed06e309bf334942c4e58cdf1a46e237911ccd7fcf9787cbc7fd0");
    }

    #[test]
    fn equal_hash_256() {
        let data1: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let data2: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let mut runtime = MockRuntime::default();
        let hash1 = runtime.hash(&data1);
        let hash2 = runtime.hash(&data2);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn not_equal_hash_256() {
        let data1: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let data2: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut runtime = MockRuntime::default();
        let hash1 = runtime.hash(&data1);
        let hash2 = runtime.hash(&data2);

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn equal_hash_512() {
        let data1: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let data2: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let mut runtime = MockRuntime::default();
        let hash1 = runtime.hash_512(&data1);
        let hash2 = runtime.hash_512(&data2);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn not_equal_hash_512() {
        let data1: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let data2: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let mut runtime = MockRuntime::default();
        let hash1 = runtime.hash_512(&data1);
        let hash2 = runtime.hash_512(&data2);

        assert_ne!(hash1, hash2);
    }
}
