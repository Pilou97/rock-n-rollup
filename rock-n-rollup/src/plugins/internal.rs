use crate::core::{FromInput, Input, Runtime};

pub trait FromInternal
where
    Self: Sized,
{
    fn from_internal(input: &[u8]) -> Result<Self, ()>;
}

pub struct Internal<T>
where
    T: FromInternal,
{
    level: u32,
    id: u32,
    payload: T,
}

impl<T: FromInternal> Internal<T> {
    pub fn level(&self) -> &u32 {
        &self.level
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

pub struct StartOfLevel;
pub struct InfoPerLevel;
pub struct EndOfLevel;

impl FromInternal for StartOfLevel {
    fn from_internal(input: &[u8]) -> Result<Self, ()> {
        match input[..] {
            [0x00, 0x01, ..] => Ok(StartOfLevel {}),
            _ => Err(()),
        }
    }
}

impl FromInternal for InfoPerLevel {
    fn from_internal(input: &[u8]) -> Result<Self, ()> {
        match input[..] {
            [0x00, 0x03, ..] => Ok(InfoPerLevel {}),
            _ => Err(()),
        }
    }
}

impl FromInternal for EndOfLevel {
    fn from_internal(input: &[u8]) -> Result<Self, ()> {
        match input[..] {
            [0x00, 0x02, ..] => Ok(EndOfLevel {}),
            _ => Err(()),
        }
    }
}

impl<T: FromInternal> FromInput<Vec<u8>> for Internal<T> {
    fn from_input<R: Runtime>(_: &mut R, input: &Input<Vec<u8>>) -> Result<Self, ()> {
        let payload = T::from_internal(&input.payload)?;
        Ok(Internal {
            level: input.level,
            id: input.id,
            payload,
        })
    }
}
