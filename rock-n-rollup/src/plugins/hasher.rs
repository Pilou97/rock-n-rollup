use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

use crate::core::Runtime;

#[derive(PartialEq, Eq, Debug)]
pub struct Blake2b512([u8; 64]);

#[derive(PartialEq, Eq, Debug)]
pub struct Blake2b256([u8; 32]);

pub trait Hasher {
    /// Hash any data to a Blake2b 256 bits
    fn hash<T: AsRef<[u8]>>(&mut self, data: &T) -> Blake2b256;

    /// Hash any data to a Blake2b 512 bits
    fn hash_512<T: AsRef<[u8]>>(&mut self, data: &T) -> Blake2b512;
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

impl<R> Hasher for R
where
    R: Runtime,
{
    fn hash<T: AsRef<[u8]>>(&mut self, data: &T) -> Blake2b256 {
        let mut hasher = Blake2bVar::new(32).unwrap();
        hasher.update(data.as_ref());
        let mut buf = [0u8; 32];
        hasher.finalize_variable(&mut buf).unwrap();
        Blake2b256(buf)
    }

    fn hash_512<T: AsRef<[u8]>>(&mut self, data: &T) -> Blake2b512 {
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
    fn hash_512() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let mut runtime = MockRuntime::default();

        let hash = runtime.hash_512(&data);
        assert_eq!(
            hash.to_string(),
            "a482fdc4e226d57674e9a9086fc79e97deb5a648922c478e6347b32815d810b1df289553cf6f501c4c230a0b0fc88b58079e7d6798ca3278ecb2ce3db67cb1ab"
        );
    }

    #[test]
    fn hash_256() {
        let data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04];
        let mut runtime = MockRuntime::default();

        let hash = runtime.hash(&data);

        assert_eq!(
            hash.to_string(),
            "28517e4cdf6c90798c1a983b03727ca7743c21a3880672429ccfc5bd15ea5f72"
        );
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
