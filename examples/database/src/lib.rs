use rock_n_rollup::{
    core::{Application, CustomRuntime},
    plugins::database::{Database, Json},
};

pub fn hello<R: Database<Json>>(rt: &mut R) {
    let data = "Hello world";

    let _ = rt.save("/data", &data.to_string());
    let data_read = rt.get::<String>("/data").unwrap().unwrap();

    assert_eq!(data, data_read);
}

#[rock_n_rollup::main]
pub fn main<R: CustomRuntime>(application: &mut Application<R: CustomRuntime>) {
    application.register(hello).run();
}
