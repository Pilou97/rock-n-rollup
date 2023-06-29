use std::collections::HashMap;

use tezos_smart_rollup_host::path::OwnedPath;

use super::constants::PREIMAGE_HASH_SIZE;

pub const MAX_MESSAGE_SIZE: usize = 4096;

#[derive(Clone)]
pub struct RawInput {
    pub level: u32,
    pub id: u32,
    pub payload: Vec<u8>,
}

#[repr(C)]
pub struct ReadInputMessageInfo {
    pub level: i32,
    pub id: i32,
}

pub trait Runtime: 'static {
    /// Print a message in the rollup stdout (if activated)
    fn write_debug(&mut self, msg: &str);

    /// Read one input
    fn next_input(&mut self) -> Option<RawInput>;

    /// Returns true if something is present under the following path
    fn store_is_present(&mut self, path: &str) -> bool;

    /// Deletes the path at the following location
    fn store_delete(&mut self, path: &str) -> Result<(), ()>;

    /// Read some data at a given path
    fn store_read(&mut self, path: &str, offset: usize, size: usize) -> Option<Vec<u8>>;

    /// Write some data at a given path
    ///
    /// TODO: this function should always be used
    /// The function stored_write and store_read should be removed and put in the Database plugin
    fn store_write(&mut self, path: &str, data: &[u8], at_offset: usize) -> Result<(), ()>;

    /// Reveal date from the reveal data directory
    fn reveal_preimage(&mut self, hash: &[u8; PREIMAGE_HASH_SIZE]) -> Result<Vec<u8>, ()>;

    /// Move the data to another path
    fn store_move(&mut self, from: &str, to: &str) -> Result<(), ()>;
}

#[derive(Default)]
pub struct KernelRuntime<R>
where
    R: tezos_smart_rollup_host::runtime::Runtime + 'static,
{
    host: R,
}

impl<R> KernelRuntime<R>
where
    R: tezos_smart_rollup_host::runtime::Runtime,
{
    pub fn new(host: R) -> Self {
        Self { host }
    }
}

impl<R> Runtime for KernelRuntime<R>
where
    R: tezos_smart_rollup_host::runtime::Runtime,
{
    fn write_debug(&mut self, msg: &str) {
        self.host.write_debug(msg)
    }

    fn store_delete(&mut self, path: &str) -> Result<(), ()> {
        let path = OwnedPath::try_from(path.to_string()).map_err(|_| ())?;

        let res = self.host.store_delete(&path);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(print!("Error store_delete")),
        }
    }

    fn store_read(&mut self, path: &str, offset: usize, size: usize) -> Option<Vec<u8>> {
        let path = OwnedPath::try_from(path.to_string()).map_err(|_| ());
        match path {
            Ok(path) => {
                let res = self.host.store_read(&path, offset, size);
                match res {
                    Ok(t) => Some(t),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    fn store_write(&mut self, path: &str, data: &[u8], at_offset: usize) -> Result<(), ()> {
        let path = OwnedPath::try_from(path.to_string()).map_err(|_| ())?;

        let res = self.host.store_write(&path, data, at_offset);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(print!("Error store_write")),
        }
    }

    fn store_move(&mut self, from: &str, to: &str) -> Result<(), ()> {
        let from = OwnedPath::try_from(from.to_string()).map_err(|_| ())?;
        let to = OwnedPath::try_from(to.to_string()).map_err(|_| ())?;

        let res = self.host.store_move(&from, &to);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(print!("Error store_move")),
        }
    }

    fn reveal_preimage(&mut self, hash: &[u8; PREIMAGE_HASH_SIZE]) -> Result<Vec<u8>, ()> {
        let payload: Vec<u8> = Vec::with_capacity(MAX_MESSAGE_SIZE);
        let mut payload: Vec<u8> = payload.to_vec();
        let res: Result<usize, _> = self.host.reveal_preimage(hash, &mut payload);
        match res {
            Ok(bytes) => Ok(bytes.to_le_bytes().to_vec()),
            Err(_) => Err(print!("Error reveal_preimage")),
        }
    }

    fn store_is_present(&mut self, path: &str) -> bool {
        let path = OwnedPath::try_from(path.to_string()).map_err(|_| ());
        match path {
            Ok(path) => {
                let res = unsafe { self.host.store_has(&path) };
                match res {
                    Ok(Some(value_type)) => match value_type {
                        tezos_smart_rollup_host::runtime::ValueType::Subtree => true,
                        tezos_smart_rollup_host::runtime::ValueType::ValueWithSubtree => true,
                        tezos_smart_rollup_host::runtime::ValueType::Value => true,
                        tezos_smart_rollup_host::runtime::ValueType::ValueWithSubtree => true,
                    },
                    Ok(None) => false, // no file
                    Err(_None) => false,
                }
            }
            Err(_) => false,
        }
    }

    fn next_input(&mut self) -> Option<RawInput> {
        let size = { self.host.read_input() };

        match size {
            Ok(Some(msg)) => Some(RawInput {
                level: msg.level,
                id: msg.id,
                payload: msg.as_ref().to_vec(),
            }),
            Ok(None) => None,
            Err(_) => None,
        }
    }
}

pub struct MockRuntime {
    stdout: Vec<String>,
    inputs: Vec<RawInput>,
    storage: HashMap<String, Vec<u8>>,
}

impl Default for MockRuntime {
    fn default() -> Self {
        Self {
            stdout: Vec::default(),
            inputs: Vec::default(),
            storage: HashMap::default(),
        }
    }
}

impl MockRuntime {
    pub fn stdout(&self) -> Vec<&str> {
        self.stdout
            .iter()
            .map(|str| str.as_str())
            .collect::<Vec<&str>>()
    }

    pub fn add_input(&mut self, input: Vec<u8>) -> &mut Self {
        let level = 0;
        let id = self.inputs.len();
        let msg = RawInput {
            level,
            id: u32::try_from(id).unwrap(),
            payload: input,
        };
        self.inputs.push(msg);
        self
    }
}

impl Runtime for MockRuntime {
    fn write_debug(&mut self, msg: &str) {
        self.stdout.push(msg.to_string());
    }

    fn next_input(&mut self) -> Option<RawInput> {
        self.inputs.pop()
    }

    fn store_is_present(&mut self, _path: &str) -> bool {
        todo!()
    }

    fn store_delete(&mut self, _path: &str) -> Result<(), ()> {
        todo!()
    }

    fn store_read(&mut self, path: &str, offset: usize, size: usize) -> Option<Vec<u8>> {
        let bytes = self.storage.get(path).cloned()?;
        if offset + size <= bytes.len() {
            let data = &bytes[offset..offset + size].to_vec();
            Some(data.clone())
        } else {
            todo!()
        }
    }

    fn store_write(&mut self, path: &str, data: &[u8], offset: usize) -> Result<(), ()> {
        let buffer = self.storage.get(path).cloned();
        match buffer {
            None => {
                self.storage.insert(path.to_string(), data.to_vec());
                println!("there");
                Ok(())
            }
            Some(mut buffer) => {
                if offset == buffer.len() {
                    println!("ici");
                    let mut data = data.to_vec();
                    buffer.append(&mut data);
                    println!("{:?}", &buffer);
                    self.storage.insert(path.to_string(), buffer.to_vec());
                    Ok(())
                } else {
                    todo!()
                }
            }
        }
    }

    fn reveal_preimage(&mut self, _hash: &[u8; PREIMAGE_HASH_SIZE]) -> Result<Vec<u8>, ()> {
        todo!()
    }

    fn store_move(&mut self, _from: &str, _to: &str) -> Result<(), ()> {
        todo!()
    }
}
