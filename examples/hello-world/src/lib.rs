use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::logger::Logger,
};

pub fn hello<L: Logger>(logger: &mut L) {
    logger.info("Hello world");
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
