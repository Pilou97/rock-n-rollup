use crate::core::{FromInput, Input, Runtime};

#[derive(Debug, PartialEq, Eq)]
pub struct Index(u32);

impl<I, S> FromInput<I, S> for Index {
    fn from_input<R: Runtime>(_: &mut R, input: &Input<I>, _: &S) -> Result<Self, ()> {
        Ok(Index(input.id))
    }
}

impl AsRef<u32> for Index {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}
