use crate::core::{FromInput, Input, Runtime};

pub struct External<T> {
    level: u32,
    id: u32,
    payload: T,
}

impl<T> External<T> {
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

impl FromInput for External<String> {
    fn from_input<R: Runtime>(_: &mut R, input: Input) -> Result<Self, ()> {
        let payload = String::from_utf8(input.payload).map_err(|_| ())?;
        Ok(External {
            level: input.level,
            id: input.id,
            payload,
        })
    }
}
