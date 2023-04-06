use crate::{Input, Runtime};

pub struct Application<'a, R, F>
where
    R: Runtime,
    F: Fn(&mut R, Input) + 'static,
{
    runtime: &'a mut R,
    transitions: Vec<Box<F>>,
}

impl<'a, R, F> Application<'a, R, F>
where
    R: Runtime,
    F: Fn(&mut R, Input),
{
    pub fn new(runtime: &'a mut R) -> Self {
        Self {
            runtime,
            transitions: Vec::default(),
        }
    }

    pub fn register(&mut self, transition: F) -> &mut Self {
        let boxed = Box::new(transition);
        self.transitions.push(boxed);
        self
    }

    pub fn run(&mut self) {
        let mut is_running = true;
        while is_running {
            let input = self.runtime.next_input();
            match input {
                None => is_running = false,
                Some(input) => {
                    self.transitions.iter().for_each(|transition| {
                        transition(self.runtime, input.clone());
                    });
                }
            }
        }
    }
}
