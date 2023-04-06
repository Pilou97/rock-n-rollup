use rock_n_rollup::{App, External, Input, Logger};

/// This function will be call each time
pub fn transition<L: Logger>(logger: &mut L, input: Input) {
    let Input { level, id, .. } = input;
    let msg = format!("The input is at level {} at index {}", level, id);
    logger.info(&msg);
}

/// This function will be call only when there is a String as an External message in the inbox
pub fn string_transition<L: Logger>(logger: &mut L, input: External<String>) {
    let msg = format!("Read message that contains the string {}", input.payload());
    logger.info(&msg);
}

#[rock_n_rollup::main]
pub fn main<Application: App>(app: &mut Application) {
    app.register(transition).register(string_transition).run();
}
