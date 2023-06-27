//use tezos_smart_rollup_host::runtime::Runtime;

use crate::core::CustomRuntime;

pub trait Logger {
    /// Logs to stdout
    ///
    /// Add an break at the end of the line
    /// If you don't want to add a break please use write_debug
    fn log(&mut self, msg: &str);

    /// Prefix the log by [INFO]
    fn info(&mut self, msg: &str);

    /// Prefix the log by [WARN]
    fn warn(&mut self, msg: &str);

    /// Prefix the log by [ERR]
    fn err(&mut self, msg: &str);
}

impl<T> Logger for T
where
    T: CustomRuntime,
{
    fn log(&mut self, msg: &str) {
        self.write_debug(&format!("{}\n", msg));
    }

    fn info(&mut self, msg: &str) {
        self.write_debug(&format!("[INFO] {}\n", msg));
    }

    fn warn(&mut self, msg: &str) {
        self.write_debug(&format!("[WARN] {}\n", msg));
    }

    fn err(&mut self, msg: &str) {
        self.write_debug(&format!("[ERR] {}\n", msg));
    }
}

#[cfg(test)]
mod tests {
    use crate::core::MockRuntime;

    use super::Logger;

    #[test]
    fn log_test() {
        let mut runtime = MockRuntime::default();
        runtime.log("Hello world");

        assert_eq!(runtime.stdout(), vec!["Hello world\n"])
    }

    #[test]
    fn info_test() {
        let mut runtime = MockRuntime::default();
        runtime.info("Hello world");

        assert_eq!(runtime.stdout(), vec!["[INFO] Hello world\n"])
    }

    #[test]
    fn warn_test() {
        let mut runtime = MockRuntime::default();
        runtime.warn("Hello world");

        assert_eq!(runtime.stdout(), vec!["[WARN] Hello world\n"])
    }

    #[test]
    fn err_test() {
        let mut runtime = MockRuntime::default();
        runtime.err("Hello world");

        assert_eq!(runtime.stdout(), vec!["[ERR] Hello world\n"])
    }
}
