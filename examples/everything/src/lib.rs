use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::logger::Logger,
    services::external::{External, FromExternal},
};

//use tezos_smart_rollup_host::runtime::Runtime;

pub enum PingPong {
    Ping,
    Pong,
}

impl FromExternal for PingPong {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        match input.as_slice() {
            [0x00] => Ok(PingPong::Ping),
            [0x01] => Ok(PingPong::Pong),
            _ => Err(()),
        }
    }
}

pub fn hello<L: Logger>(logger: &mut L, _ping_pong: External<PingPong>) {
    logger.log("Hello world");
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
