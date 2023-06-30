use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::{
        database::{Database, Json},
        logger::Logger,
    },
    services::external::{External, FromExternal},
};

pub struct Decrement {}

impl FromExternal for Decrement {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        match input.as_slice() {
            [0x00] => Ok(Decrement {}),
            _ => Err(()),
        }
    }
}

pub fn descrement_entrypoint<R: Runtime>(rt: &mut R, _: External<Decrement>) {
    rt.log("descrement counter");
}

pub fn counter<L: Logger + Database<Json>>(rt: &mut L, _: External<Decrement>) {
    rt.log("A counter just appeared");

    let counter = rt.get::<i32>("/data").ok().unwrap();
    counter.unwrap_or(32);

    let _data = rt.save("/data", &counter.unwrap());
    let read_counter = rt.get::<i32>("/data").unwrap().unwrap();
    assert_eq!(counter.unwrap(), read_counter);
}

#[rock_n_rollup::main]
pub fn main<R>(application: &mut Application<R>)
where
    R: rock_n_rollup::core::Runtime,
{
    application
        .register(descrement_entrypoint)
        .register(counter)
        .run()
}
