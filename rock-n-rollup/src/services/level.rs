use crate::core::{FromInput, Input, Runtime};

#[derive(Debug, PartialEq, Eq)]
pub struct Level(u32);

impl<I, S> FromInput<I, S> for Level {
    fn from_input<R: Runtime>(_: &mut R, input: &Input<I>, _: &S) -> Result<Self, ()> {
        Ok(Level(input.level))
    }
}

impl AsRef<u32> for Level {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
