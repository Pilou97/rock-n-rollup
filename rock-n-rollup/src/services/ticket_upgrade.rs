use crate::{
    core::{FromInput, Input, IntoService, Runtime, Service},
    plugins::{
        dac::{Dac, PreimageHash},
        installer::Installer,
        logger::Logger,
    },
};
use tezos_smart_rollup_encoding::michelson::{ticket::Ticket, MichelsonBytes};

use super::internal::{Internal, Transfer};

#[derive(Clone)]
pub struct TicketUpgrade {
    from_contract: String,
}

impl TicketUpgrade {
    pub fn new(addr: &str) -> Self {
        Self {
            from_contract: addr.to_string(),
        }
    }
}

impl FromInput<Vec<u8>, TicketUpgrade> for TicketUpgrade {
    fn from_input<R: Runtime>(
        _: &mut R,
        _: &Input<Vec<u8>>,
        state: &TicketUpgrade,
    ) -> Result<Self, ()> {
        Ok(state.clone())
    }
}

fn upgrade_on_ticket<R: Logger + Dac + Installer>(
    rt: &mut R,
    transfer: Internal<Transfer<Ticket<MichelsonBytes>>>,
    state: TicketUpgrade,
) {
    let transfer = transfer.payload();
    if &state.from_contract == transfer.sender() {
        rt.info("Trying to upgrade");
        let ticket = transfer.payload();
        let MichelsonBytes(data) = ticket.contents();
        let root_hash = PreimageHash::try_from(data).unwrap();
        let new_kernel = rt.read_from_dac(&root_hash).unwrap();
        rt.install(&new_kernel).unwrap();
        // match new_kernel {
        //     Err(_) => rt.err("Error when deserializing message from the DAC"),
        //     Ok(new_kernel) => rt.install(new_kernel),
        // }
    }
}

impl<R> IntoService<R, Vec<u8>, TicketUpgrade> for TicketUpgrade
where
    R: Runtime,
{
    fn into_service(self) -> Service<R, Vec<u8>, Self> {
        let mut service = Service::<R, Vec<u8>, Self>::new(self);
        service.register(upgrade_on_ticket);
        service
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{Application, MockRuntime};

    use super::TicketUpgrade;

    #[test]
    fn test_2() {
        let mut runtime = MockRuntime::default();
        let mut application = Application::new(&mut runtime);

        application
            .service(TicketUpgrade::new("KT1CM7YzV9PaKX8u7HKM7vgey3Bbn6ZpZvWZ"))
            .run();
    }
}
