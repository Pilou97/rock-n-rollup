#[link(wasm_import_module = "smart_rollup_core")]
extern "C" {
    /// Does nothing. Does not check the correctness of its argument.
    pub fn write_debug(src: *const u8, num_bytes: usize);
}

pub trait Runtime {
    /// Print a message in the rollup stdout (if activated)
    fn write_debug(&mut self, msg: &str);
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
}
