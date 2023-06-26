use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::{
        database::{Database, Json},
        logger::Logger,
    },
    services::{
        external::{External, FromExternal},
        internal::{InfoPerLevel, Internal, StartOfLevel},
        ticket_upgrade::TicketUpgrade,
    },
};

pub fn on_start<R: Runtime>(rt: &mut R, _: Internal<StartOfLevel>) {
    rt.write_debug("Hello start of the inbox");
}

pub struct Decrement {}

impl FromExternal for Decrement {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        match input.as_slice() {
            [0x00] => Ok(Decrement {}),
            _ => Err(()),
        }
    }
}

pub fn increment<R: Runtime>(rt: &mut R, _: Internal<InfoPerLevel>) {}

pub fn decrement<R: Runtime>(rt: &mut R, _: External<Decrement>) {}

// pub fn hello<L: Logger + Database<Json>>(rt: &mut L) {
//     let counter = 32;
//     let a = rt.save("/data", &counter);
//     // let mut bytes = Vec::default();
//     // bytes.push(coutner.to_be_bytes())
//     // let path = RefPath::assert_from("...");
//     // host.store_write(path, bytes, 0, 0);
//     let read_counter = rt.get::<i32>("/data");

//     rt.log("Hello world");
// }

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application
        .register(on_start)
        .register(increment)
        .register(decrement)
        .service(TicketUpgrade::new("KT1...."))
        .run();
}

#[cfg(test)]
mod tests {
    use rock_n_rollup::core::{Application, MockRuntime};

    use crate::main;

    #[test]
    fn test() {
        let mut runtime = MockRuntime::default();
        runtime.add_input(Vec::default());

        let mut application = Application::new(&mut runtime);

        let () = main(&mut application);

        assert_eq!(runtime.stdout(), vec!["Hello world\n"]);
    }
}
