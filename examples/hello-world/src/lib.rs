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
