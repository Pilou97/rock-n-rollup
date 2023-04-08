use crate::core::Runtime;
use serde::{de::DeserializeOwned, Serialize};

pub trait Database {
    fn read<'de, D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned;

    fn write<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize;

    fn delete(&mut self, path: &str) -> Result<(), ()>;
    fn is_present(&mut self, path: &str) -> bool;
}

impl<T> Database for T
where
    T: Runtime,
{
    fn read<D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned,
    {
        let payload = self.store_read(path);
        match payload {
            None => Ok(None),
            Some(payload) => {
                let data = bincode::deserialize(&payload);
                match data {
                    Ok(data) => Ok(Some(data)),
                    Err(_) => Ok(None),
                }
            }
        }
    }

    fn write<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize,
    {
        let payload = bincode::serialize(data).map_err(|_| ())?;
        self.store_write(path, &payload).map(|_| data)
    }

    fn delete(&mut self, path: &str) -> Result<(), ()> {
        self.store_delete(path)
    }

    fn is_present(&mut self, path: &str) -> bool {
        self.store_is_present(path)
    }
}
