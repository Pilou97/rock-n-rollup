use rock_n_rollup::{
    core::Application, plugins::logger::Logger, services::ticket_upgrade::TicketUpgrade,
};

fn transition<R: Logger>(rt: &mut R) {
    rt.log("world hello");
}

#[rock_n_rollup::main]
pub fn main<R: rock_n_rollup::core::Runtime>(application: &mut Application<R>) {
    application
        .service(TicketUpgrade::new("KT1CM7YzV9PaKX8u7HKM7vgey3Bbn6ZpZvWZ"))
        .register(transition)
        .run();
}
