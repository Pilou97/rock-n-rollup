use super::Input;

pub trait Middleware {
    fn upgrade(&self, input: Input) -> Result<Input, ()>;
}
