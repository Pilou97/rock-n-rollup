use rock_n_rollup::{Application, Input, Logger, Runtime};

pub fn transition<R: Runtime>(runtime: &mut R, input: Input) {
    runtime.info(&format!(
        "The input is at level {} at index {}",
        input.level, input.id
    ));
}

#[rock_n_rollup::main]
pub fn main<R: Runtime + Logger + 'static>(runtime: &mut R) {
    let mut app = Application::new(runtime);
    app.register(transition).run();
}
