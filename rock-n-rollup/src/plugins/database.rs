use serde::{de::DeserializeOwned, Serialize};

use crate::core::Runtime;

/// Database to read and write data from the durable storage
///
///
/// Each field of the struct will be written in the
pub trait Database {
    /// Get the data from the database at a given path
    fn get<D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned;

    /// Save the data in the database at a given path
    fn save<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize;
}

impl<R> Database for R
where
    R: Runtime,
{
    fn get<D>(&mut self, path: &str) -> Result<Option<D>, ()>
    where
        D: DeserializeOwned,
    {
        let bytes = self.store_read(path);
        match bytes {
            None => Ok(None),
            Some(bytes) => {
                let decoded = bincode::deserialize(&bytes).map_err(|_| ())?;
                Ok(Some(decoded))
            }
        }
    }

    fn save<'a, D>(&mut self, path: &str, data: &'a D) -> Result<&'a D, ()>
    where
        D: Serialize,
    {
        let encoded: Vec<u8> = bincode::serialize(data).map_err(|_| ())?;

        match self.store_write(path, &encoded) {
            Ok(_) => Ok(data),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::MockRuntime;

    use super::Database;

    #[test]
    fn test() {
        let mut runtime = MockRuntime::default();
        let data = "Hello world".to_string();

        let _ = runtime.save("/greet", &data);
        let greetings = runtime.get::<String>("/greet").unwrap().unwrap();

        assert_eq!(greetings, data)
    }
}
