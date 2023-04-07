use crate::core::{Input, Middleware};

pub struct MagicByte {
    magic_byte: u8,
}

impl MagicByte {
    pub fn new(magic_byte: u8) -> Self {
        MagicByte { magic_byte }
    }
}

impl Middleware for MagicByte {
    fn upgrade(&self, input: Input) -> Result<Input, ()> {
        match input.payload[..] {
            [0x01, magic_byte, ..] => match magic_byte == self.magic_byte {
                true => {
                    // The best thing to do is to return [0x01, and the rest of the message without the magic byte]
                    // So that other plugins will continue to work normally
                    // Because the External one is deserializing everything after 0x01
                    // Remove the magic_byte will preserve its behavior
                    // TODO: test to make sure there is border effect between plugins
                    let payload = std::iter::once(&0x01)
                        .chain(input.payload.iter().skip(2))
                        .copied()
                        .collect::<Vec<u8>>();

                    Ok(Input {
                        id: input.id,
                        level: input.level,
                        payload,
                    })
                }
                false => Err(()),
            },
            _ => Ok(input),
        }
    }
}
