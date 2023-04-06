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
}

pub trait Runtime {
    /// Print a message in the rollup stdout (if activated)
    fn write_debug(&mut self, msg: &str);

    /// Read one input
    fn next_input(&mut self) -> Option<Input>;
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
}
