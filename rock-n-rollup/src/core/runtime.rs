pub const MAX_MESSAGE_SIZE: usize = 4096;

#[derive(Clone)]
pub struct Input {
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

}

pub trait Runtime {
    /// Print a message in the rollup stdout (if activated)
    fn write_debug(&mut self, msg: &str);

    /// Read one input
    fn next_input(&mut self) -> Option<Input>;

    /// Returns true if something is present under the following path
    fn is_present(&mut self, path: &str) -> bool;

    /// Deletes the path at the following location
    fn delete(&mut self, path: &str) -> Result<(), ()>;

    /// Read some data at a given path
    fn read(&mut self, path: &str) -> Option<Vec<u8>>;

    /// Write some data at a given path
    fn write(&mut self, path: &str, data: &Vec<u8>) -> Result<(), ()>;
}

pub struct KernelRuntime {}

impl KernelRuntime {
    pub fn new() -> Self {
        KernelRuntime {}
    }
}

impl Runtime for KernelRuntime {
    fn write_debug(&mut self, msg: &str) {
        unsafe {
            write_debug(msg.as_ptr(), msg.len());
        }
    }

    fn next_input(&mut self) -> Option<Input> {
        let mut payload = Vec::with_capacity(MAX_MESSAGE_SIZE as usize);

        // Placeholder values
        let mut message_info = ReadInputMessageInfo { level: 0, id: 0 };

        let size = unsafe { read_input(&mut message_info, payload.as_mut_ptr(), MAX_MESSAGE_SIZE) };

        if size == 0 {
            None
        } else {
            unsafe { payload.set_len(size as usize) };
            Some(Input {
                level: message_info.level as u32,
                id: message_info.id as u32,
                payload,
            })
        }
    }

    fn is_present(&mut self, path: &str) -> bool {
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

    fn delete(&mut self, path: &str) -> Result<(), ()> {
        let ptr = path.as_ptr();
        let res = unsafe { store_delete(ptr, path.len()) };
        match res {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        if !self.is_present(path) {
            return None;
        }

        let ptr = path.as_ptr();
        let path_len = path.len();
        let usize_size = std::mem::size_of::<usize>();

        let size = unsafe {
            let mut buffer = Vec::with_capacity(usize_size);
            let dst = buffer.as_mut_ptr();
            let _ = store_read(ptr, path_len, 0, dst, usize_size);
            buffer.set_len(usize_size);
            let be_bytes = buffer.try_into().unwrap(); // Should be ok
            usize::from_be_bytes(be_bytes)
        };

        let mut buffer = Vec::with_capacity(size);
        let dst = buffer.as_mut_ptr();
        unsafe {
            let _ = store_read(ptr, path_len, usize_size, dst, size);
            buffer.set_len(size);
        }

        Some(buffer)
    }

    fn write(&mut self, path: &str, data: &Vec<u8>) -> Result<(), ()> {
        let ptr = path.as_ptr();
        let data = data.clone();
        let length = data.len();

        let mut length_bytes = length.to_be_bytes().to_vec();

        let (length_res, data_res) = unsafe {
            // First we wrote the size of the data, this size has an known size
            let res1 = store_write(
                ptr,
                path.len(),
                0,
                length_bytes.as_mut_ptr(),
                length_bytes.len(),
            );

            // Then we write the data
            let res2 = store_write(
                ptr,
                path.len(),
                length_bytes.len(),
                data.as_ptr(),
                data.len(),
            );
            (res1, res2)
        };

        match (length_res, data_res) {
            (0, 0) => Ok(()),
            _ => Err(()),
        }
    }
}
