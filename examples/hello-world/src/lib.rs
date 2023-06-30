use rock_n_rollup::{core::Application, plugins::logger::Logger};

pub fn hello<L: Logger>(logger: &mut L) {
    logger.log("Hello world");
}

#[rock_n_rollup::main]
pub fn main<R: rock_n_rollup::core::Runtime>(application: &mut Application<R>) {
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

        main(&mut application);

        assert_eq!(runtime.stdout(), vec!["Hello world\n"]);
    }
}
