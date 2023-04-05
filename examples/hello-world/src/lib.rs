use rock_n_rollup::Logger;

#[rock_n_rollup::main]
pub fn main<L: Logger>(logger: &mut L) {
    logger.info("Hello world!");
}
