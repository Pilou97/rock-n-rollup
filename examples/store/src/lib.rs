use rock_n_rollup::{
    core::{App, Runtime},
    plugins::{external::External, logger::Logger},
};

/// This function will be call each time
pub fn transition<R: Runtime + Logger>(rt: &mut R, _: External<String>) {
    let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    let res = rt.store_write("/test", &data);

    if let Ok(()) = res {
        rt.info("Write is a success");
        if let Some(read) = rt.store_read("/test") {
            if read == data {
                rt.info("Read is a success");
                if let Ok(()) = rt.store_delete("/test") {
                    rt.info("Delete is a success");
                    let read = rt.store_read("/test");
                    if let None = read {
                        rt.info("Data is not there");
                    }
                } else {
                    rt.info("Data is not deleted");
                }
            }
        }
    }
}

#[rock_n_rollup::main]
pub fn main<Application: App>(application: &mut Application) {
    application.register(transition).run();
}
