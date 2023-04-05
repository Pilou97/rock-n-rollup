use crate::Runtime;

pub trait Logger {
    fn info(&mut self, msg: &str);
}

impl<T> Logger for T
where
    T: Runtime,
{
    fn info(&mut self, msg: &str) {
        self.write_debug(&format!("{}\n", msg));
    }
}
