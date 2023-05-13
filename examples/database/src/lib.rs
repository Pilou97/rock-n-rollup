use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::database::Database,
};

pub fn hello<R: Database>(rt: &mut R) {
    let data = "Hello world";

    let _ = rt.save("/data", &data.to_string());
    let data_read = rt.get::<String>("/data").unwrap().unwrap();

    assert_eq!(data, data_read);
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
