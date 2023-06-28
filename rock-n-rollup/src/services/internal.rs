use tezos_smart_rollup_encoding::{
    inbox::{InboxMessage, InternalInboxMessage},
    michelson::Michelson,
};

use crate::core::{FromInput, Input, Runtime};
//use tezos_smart_rollup_host::runtime::Runtime;

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

/// TODO: better types or use tht tezos sdk
pub struct Transfer<M>
where
    M: Michelson,
{
    payload: M,
    sender: String,
    source: String,
    destination: String,
}

impl<M> Transfer<M>
where
    M: Michelson,
{
    pub fn payload(&self) -> &M {
        &self.payload
    }

    pub fn sender(&self) -> &String {
        &self.sender
    }

    pub fn source(&self) -> &String {
        &self.source
    }

    pub fn destination(&self) -> &String {
        &self.destination
    }
}

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

impl<M> FromInternal for Transfer<M>
where
    M: Michelson,
{
    fn from_internal(input: &[u8]) -> Result<Self, ()> {
        let (_, msg) = InboxMessage::<M>::parse(input).map_err(|_| ())?;

        match msg {
            InboxMessage::Internal(InternalInboxMessage::Transfer(t)) => {
                let payload = t.payload;
                let sender = t.sender.to_base58_check();
                let source = t.source.to_b58check();
                let destination = t.destination.to_b58check();

                Ok(Transfer {
                    payload,
                    sender,
                    source,
                    destination,
                })
            }
            _ => Err(()),
        }
    }
}

impl<T: FromInternal, S> FromInput<Vec<u8>, S> for Internal<T> {
    fn from_input<R: Runtime>(_: &mut R, input: &Input<Vec<u8>>, _: &S) -> Result<Self, ()> {
        let payload = T::from_internal(&input.payload)?;
        Ok(Internal {
            level: input.level,
            id: input.id,
            payload,
        })
    }
}
