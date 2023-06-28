use std::mem::size_of;

use serde::{de::DeserializeOwned, Serialize};

use crate::core::Runtime;

//use tezos_smart_rollup_host::runtime::Runtime;

pub trait Backend {
    fn to_bytes<P>(data: &P) -> Result<Vec<u8>, ()>
    where
        P: Serialize;

    fn from_bytes<P>(bytes: &[u8]) -> Result<P, ()>
    where
        P: DeserializeOwned;
}

/// Json backend for the database
pub struct Json {}

impl Backend for Json {
    fn to_bytes<P>(data: &P) -> Result<Vec<u8>, ()>
    where
        P: Serialize,
    {
        serde_json_wasm::to_vec(data).map_err(|_| ())
    }

    fn from_bytes<P>(bytes: &[u8]) -> Result<P, ()>
    where
        P: DeserializeOwned,
    {
        serde_json_wasm::from_slice(bytes).map_err(|_| ())
    }
}

/// Bincode backend for the database
pub struct Bincode {}

impl Backend for Bincode {
    fn to_bytes<P>(data: &P) -> Result<Vec<u8>, ()>
    where
        P: Serialize,
    {
        bincode::serialize(data).map_err(|_| ())
    }

    fn from_bytes<P>(bytes: &[u8]) -> Result<P, ()>
    where
        P: DeserializeOwned,
    {
        bincode::deserialize(bytes).map_err(|_| ())
    }
}

/// Database to read and write data from the durable storage
///
///
/// Each field of the struct will be written in the
pub trait Database<B>
where
    B: Backend,
{
    /// Get the data from the database at a given path
    fn get<D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned;

    /// Save the data in the database at a given path
    fn save<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize;
}

impl<R, B> Database<B> for R
where
    R: Runtime,
    B: Backend,
{
    fn get<D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned,
    {
        // The n first bytes represent the size
        let usize_size = size_of::<usize>();
        let size = self
            .store_read(path, 0, usize_size)
            .unwrap_or_default()
            .try_into()
            .map_err(|_| ())?;
        let size = usize::from_be_bytes(size);

        println!("size to read: {}", size);

        let bytes = self.store_read(path, usize_size, size);

        match bytes {
            None => Ok(None),
            Some(bytes) => {
                let decoded = B::from_bytes(&bytes)?;
                Ok(Some(decoded))
            }
        }
    }

    fn save<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize,
    {
        let bytes = B::to_bytes(data)?;
        let size = bytes.len();
        let usize_size = size_of::<usize>();
        let size_bytes = size.to_be_bytes();

        // Let's write the size at the beginning
        self.store_write(path, &size_bytes, 0).map_err(|_| ())?;

        match self.store_write(path, &bytes, usize_size) {
            Ok(_) => Ok(data),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::println;

    use crate::{core::MockRuntime, plugins::database::Bincode};

    use super::{Backend, Database, Json};

    fn test_backend<B: Backend>() {
        let mut runtime = MockRuntime::default();
        let data = "Hello world".to_string();

        let _ = <MockRuntime as Database<B>>::save(&mut runtime, "/greet", &data).unwrap();
        println!("saved");
        let greetings = <MockRuntime as Database<B>>::get::<String>(&mut runtime, "/greet")
            .unwrap()
            .unwrap();

        assert_eq!(greetings, data)
    }

    #[test]
    fn test_json() {
        test_backend::<Json>()
    }

    #[test]
    fn test_bincode() {
        test_backend::<Bincode>()
    }
}
