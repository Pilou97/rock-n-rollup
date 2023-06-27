use std::collections::HashMap;

use tezos_smart_rollup_host::{path::RefPath, runtime::Runtime};

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

#[link(wasm_import_module = "smart_rollup_core")]
extern "C" {
    /// Does nothing. Does not check the correctness of its argument.
    pub fn write_debug(src: *const u8, num_bytes: usize);

    pub fn read_input(
        message_info: *mut ReadInputMessageInfo,
        dst: *mut u8,
        max_bytes: usize,
    ) -> i32;

    /// Returns
    /// - 0 the key is missing
    /// - 1 only a file is stored under the path
    /// - 2 only directories under the path
    /// - 3 both a file and directories
    pub fn store_has(path: *const u8, path_len: usize) -> i32;

    /// Returns 0 in case of success, or an error code
    pub fn store_delete(path: *const u8, path_len: usize) -> i32;

    /// Returns the number of bytes written to the durable storage
    /// (should be equal to `num_bytes`, or an error code).
    pub fn store_read(
        path: *const u8,
        path_len: usize,
        offset: usize,
        dst: *mut u8,
        num_bytes: usize,
    ) -> i32;

    /// Returns 0 in case of success, or an error code.
    pub fn store_write(
        path: *const u8,
        path_len: usize,
        offset: usize,
        src: *const u8,
        num_bytes: usize,
    ) -> i32;

    /// Returns the number of bytes written at `dst`, or an error code.
    pub fn reveal_preimage(
        hash_addr: *const u8,
        hash_size: u8,
        dst: *mut u8,
        max_bytes: usize,
    ) -> i32;

    /// Returns 0 in case of success, or an error code.
    pub fn store_move(
        src_path: *const u8,
        scr_path_len: usize,
        dst_path: *const u8,
        dst_path_len: usize,
    ) -> i32;
}

pub trait CustomRuntime {
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
    R: Runtime,
{
    host: R,
}

impl<R> KernelRuntime<R>
where
    R: Runtime,
{
    pub fn new(host: R) -> Self {
        Self { host }
    }
}

impl<R> CustomRuntime for KernelRuntime<R>
where
    R: Runtime,
{
    fn write_debug(&mut self, msg: &str) {
        self.host.write_debug(msg)
    }

    fn store_delete(&mut self, path: &str) -> Result<(), ()> {
        let path = RefPath::assert_from(path.as_bytes());

        let res = self.host.store_delete(&path);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(print!("Error store_delete")),
        }
    }

    fn store_read(&mut self, path: &str, offset: usize, size: usize) -> Option<Vec<u8>> {
        let path = RefPath::assert_from(path.as_bytes());

        let res = self.host.store_read(&path, offset, size);
        match res {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    fn store_write(&mut self, path: &str, data: &[u8], at_offset: usize) -> Result<(), ()> {
        let path = RefPath::assert_from(path.as_bytes());
        let res = self.host.store_write(&path, data, at_offset);
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(print!("Error store_write")),
        }
    }

    fn store_move(&mut self, from: &str, to: &str) -> Result<(), ()> {
        let from = RefPath::assert_from(from.as_bytes());
        let to = RefPath::assert_from(to.as_bytes());

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
        let ptr = path.as_ptr();
        let res = unsafe { store_has(ptr, path.len()) };
        match res {
            0 => false, // No file
            1 => true,  // Only file
            2 => true,  // Only directory
            3 => true,  // Directory + File
            _ => false,
        }
    }

    fn next_input(&mut self) -> Option<RawInput> {
        let mut payload = Vec::with_capacity(MAX_MESSAGE_SIZE);

        // Placeholder values
        let mut message_info = ReadInputMessageInfo { level: 0, id: 0 };

        let size = unsafe { read_input(&mut message_info, payload.as_mut_ptr(), MAX_MESSAGE_SIZE) };

        if size == 0 {
            None
        } else {
            unsafe { payload.set_len(size as usize) };
            Some(RawInput {
                level: message_info.level as u32,
                id: message_info.id as u32,
                payload,
            })
        }
    }
}

/* Q: modify
#[derive(Default)]
pub struct KernelRuntime {}*/

/// First: rename Runtime into CustomRuntime
/// Second step:
/// KernelRuntime<'a, Rt: impl Runtime> { // The one from tezos-smart-rollup
///    runtime: &'a, mut Rt
/// }
/// Third step:
/// Adapt the following implementation: impl CustomRuntime for KernelRuntime
///
/// Last step: remove code starting line 21

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

impl CustomRuntime for MockRuntime {
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
