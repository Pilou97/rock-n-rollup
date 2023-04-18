use crate::core::{FromInput, Input, Runtime};

pub trait FromExternal
where
    Self: Sized,
{
    fn from_external(input: Vec<u8>) -> Result<Self, ()>;
}

pub struct External<T>
where
    T: FromExternal,
{
    level: u32,
    id: u32,
    payload: T,
}

impl<T> External<T>
where
    T: FromExternal,
{
    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }
}

impl<T> FromInput<Vec<u8>> for External<T>
where
    T: FromExternal,
{
    fn from_input<R: Runtime>(_: &mut R, input: &Input<Vec<u8>>) -> Result<Self, ()> {
        // First we need to make sure it starts by 0x01
        match input.payload[..] {
            [0x01, ..] => {
                let payload = input.payload.iter().skip(1).copied().collect::<Vec<u8>>();
                let payload = T::from_external(payload)?;
                Ok(External {
                    level: input.level,
                    id: input.id,
                    payload,
                })
            }
            _ => Err(()),
        }
    }
}

impl FromExternal for String {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        String::from_utf8(input).map_err(|_| ())
    }
}
