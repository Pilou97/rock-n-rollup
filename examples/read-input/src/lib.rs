use rock_n_rollup::{App, Input, Logger};

pub fn transition<L: Logger>(logger: &mut L, input: Input) {
    let Input { level, id, .. } = input;
    let msg = format!("The input is at level {} at index {}", level, id);
    logger.info(&msg);
}

#[rock_n_rollup::main]
pub fn main<Application: App>(app: &mut Application) {
    app.register(transition).run();
}
