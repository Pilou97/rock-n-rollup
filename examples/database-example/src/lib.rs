use rock_n_rollup::{
    core::App,
    plugins::{database::Database, external::External, logger::Logger},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    firstname: String,
    lastname: String,
}

/// This function will be call each time
pub fn transition<R: Logger + Database>(rt: &mut R, _: External<String>) {
    let path = format!("/user/{}", "2");

    let user = User {
        firstname: "Ben".to_string(),
        lastname: "Kenobi".to_string(),
    };

    if let Ok(_) = rt.write(&path, &user) {
        rt.info("User is saved in store");

        if let Ok(Some(_)) = rt.read::<User>(&path) {
            rt.info("User is retrieved from store")
        }
    } else {
        rt.info("User is not saved");
    }
}

#[rock_n_rollup::main]
pub fn main<Application: App>(application: &mut Application) {
    application.register(transition).run();
}
