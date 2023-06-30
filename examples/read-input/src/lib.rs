use rock_n_rollup::{
    core::{
        michelson::{ticket::Ticket, MichelsonBytes},
        Application, Input,
    },
    plugins::logger::Logger,
    services::{
        external::External,
        internal::{EndOfLevel, InfoPerLevel, Internal, StartOfLevel, Transfer},
    },
};

/// This function will be call each time
pub fn transition<L: Logger>(logger: &mut L, input: Input<Vec<u8>>) {
    let Input { level, id, .. } = input;
    let msg = format!("The input is at level {} at index {}", level, id);
    logger.info(&msg);
}

/// This function will be call only when there is a String as an External message in the inbox
pub fn string_transition<L: Logger>(logger: &mut L, input: External<String>) {
    let msg = format!("Read message that contains the string {}", input.payload());
    logger.info(&msg);
}

/// This function is only executed on on the StartOfLevel message
pub fn start_of_level_transition<L: Logger>(logger: &mut L, _: Internal<StartOfLevel>) {
    logger.info("Start of level");
}

/// This function is only executed on on the InfoPerLevel message
pub fn info_per_level_transition<L: Logger>(logger: &mut L, _: Internal<InfoPerLevel>) {
    logger.info("Info per level");
}

/// This function is only executed on on the EndOfLevel message
pub fn end_of_level_transition<L: Logger>(logger: &mut L, _: Internal<EndOfLevel>) {
    logger.info("End of level");
}

pub fn ticket_transition<L: Logger>(
    logger: &mut L,
    ticket: Internal<Transfer<Ticket<MichelsonBytes>>>,
) {
    logger.info("A ticket just appeared");
    let transfer = ticket.payload();
    let destination = transfer.destination();
    let _ticket = transfer.payload();
    let source = transfer.source();
    let sender = transfer.sender();

    logger.info(destination);
    logger.info(sender);
    logger.info(source);
}

#[rock_n_rollup::main]
pub fn main<R: rock_n_rollup::core::Runtime>(application: &mut Application<R>) {
    application
        // .register(transition)
        // .register(string_transition)
        // .register(start_of_level_transition)
        // .register(info_per_level_transition)
        // .register(end_of_level_transition)
        .register(ticket_transition)
        .run();
}
