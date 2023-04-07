use rock_n_rollup::{
    core::App,
    plugins::{external::External, logger::Logger, magic_byte::MagicByte},
};

/// This function will be call each time when the magic byte is matched
pub fn transition<L: Logger>(logger: &mut L, _: External<String>) {
    logger.info("Here is a message that respect the magic byte constraint")
}

#[rock_n_rollup::main]
pub fn main<Application: App>(app: &mut Application) {
    app.wrap(MagicByte::new(0x42)).register(transition).run();
}
