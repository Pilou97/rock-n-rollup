use rock_n_rollup::{Application, Input, Logger, Runtime};

pub fn transition<L: Logger>(logger: &mut L, input: Input) {
    let msg = format!(
        "Processing the input of level {} at index {} ",
        input.level, input.id
    );
    logger.info(&msg);
}

#[rock_n_rollup::main]
pub fn main<R: Runtime + Logger + 'static>(runtime: &mut R) {
    let mut app = Application::new(runtime);
    app.register(transition).run();
}
