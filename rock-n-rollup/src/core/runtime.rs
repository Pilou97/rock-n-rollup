use std::{collections::HashMap, println};

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
    /// (should be equal to `num_bytes`, or an error code.
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
pub struct KernelRuntime {}

/// First: rename Runtime into CustomRuntime
/// Second step:
/// KernelRuntime<'a, Rt: impl Runtime> { // The one from tezos-smart-rollup
///    runtime: &'a, mut Rt
/// }
/// Third step:
/// Adapt the following implementation: impl CustomRuntime for KernelRuntime
///
/// Last step: remove code starting line 21

impl Runtime for KernelRuntime {
    fn write_debug(&mut self, msg: &str) {
        // self.runtime.write_debug(msg);
        // Ok(())
    }

    fn next_input(&mut self) -> Option<RawInput> {
        let mut payload = Vec::with_capacity(MAX_MESSAGE_SIZE as usize);

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

    fn store_delete(&mut self, path: &str) -> Result<(), ()> {
        let ptr = path.as_ptr();
        let res = unsafe { store_delete(ptr, path.len()) };
        match res {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    fn store_read(&mut self, path: &str, offset: usize, size: usize) -> Option<Vec<u8>> {
        if !self.store_is_present(path) {
            return None;
        }

        let ptr = path.as_ptr();
        let path_len = path.len();
        let mut buffer = Vec::with_capacity(size);
        let dst = buffer.as_mut_ptr();
        unsafe {
            let _ = store_read(ptr, path_len, offset, dst, size);
            buffer.set_len(size);
        }

        Some(buffer)
    }

    fn store_write(&mut self, path: &str, data: &[u8], at_offset: usize) -> Result<(), ()> {
        let res = unsafe {
            let path_len = path.len();
            let path = path.as_ptr();
            let num_bytes = data.len();
            let src = data.as_ptr();
            store_write(path, path_len, at_offset, src, num_bytes)
        };
        match res {
            0 => Ok(()),
            err => {
                self.write_debug(&format!("error store_write_raw: {}\n", err));
                Err(())
            }
        }
    }
    fn reveal_preimage(&mut self, hash: &[u8; PREIMAGE_HASH_SIZE]) -> Result<Vec<u8>, ()> {
        let max_size = 4096;
        let mut payload = Vec::with_capacity(MAX_MESSAGE_SIZE as usize);

        let u8_size = u8::try_from(PREIMAGE_HASH_SIZE).unwrap();

        unsafe {
            let size = reveal_preimage(hash.as_ptr(), u8_size, payload.as_mut_ptr(), max_size);
            if size < 0 {
                Err(())
            } else {
                let size = usize::try_from(size).unwrap();
                payload.set_len(size);
                Ok(payload)
            }
        }
    }

    fn store_move(&mut self, from: &str, to: &str) -> Result<(), ()> {
        let res = unsafe { store_move(from.as_ptr(), from.len(), to.as_ptr(), to.len()) };
        match res {
            0 => Ok(()),
            _ => Err(()),
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
