use super::{
    service::{Input, IntoTransition, Runnable, Service},
    FromRawInput, IntoService,
};

use crate::core::runtime::Runtime;
//use tezos_smart_rollup_host::runtime::Runtime;

pub struct Application<'a, R: Runtime>
where
    R: Runtime,
{
    runtime: &'a mut R,
    services: Vec<Box<dyn Runnable<R>>>,
    base: Service<R, Vec<u8>, ()>,
}

impl<'a, R: Runtime + 'static> Application<'a, R> {
    pub fn register<F, Marker>(&mut self, transition: F) -> &mut Self
    where
        F: IntoTransition<R, Vec<u8>, (), Marker> + 'static,
    {
        self.base.register(transition);
        self
    }

    pub fn add_guard(&mut self, guard: fn(&mut R, &Input<Vec<u8>>) -> bool) -> &mut Self {
        self.base.add_guard(guard);
        self
    }

    pub fn service<P, S>(&mut self, service: impl IntoService<R, P, S> + 'static) -> &mut Self
    where
        P: FromRawInput + 'static,
        S: 'static,
    {
        let service = service.into_service();

        let boxed = Box::new(service);
        self.services.push(boxed);
        self
    }

    pub fn run(&mut self) {
        let mut is_running = true;
        while is_running {
            let input = self.runtime.next_input();
            match input {
                None => is_running = false,
                Some(input) => {
                    self.base.run(self.runtime, input.clone());

                    self.services.iter_mut().for_each(|service| {
                        println!("service 1");
                        service.run(self.runtime, input.clone())
                    });
                }
            }
        }
    }
}

impl<'a, R> Application<'a, R>
where
    R: Runtime,
{
    pub fn new(runtime: &'a mut R) -> Self {
        Self {
            runtime,
            services: Vec::default(),
            base: Service::<R, Vec<u8>, ()>::new(()),
        }
    }
}
