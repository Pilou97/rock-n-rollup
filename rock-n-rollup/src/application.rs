use crate::{Input, Runtime};

trait FromExternal {
    fn from_external<R: Runtime>(runtime: &mut R, input: Input) -> Self;
}

impl FromExternal for Input {
    fn from_external<R: Runtime>(_: &mut R, input: Input) -> Self {
        input
    }
}

pub trait IntoHandler<R, T>
where
    R: Runtime,
{
    fn make_handler(self) -> Box<dyn FnMut(&mut R, Input)>;
}

impl<F, R> IntoHandler<R, ()> for F
where
    R: Runtime,
    F: Fn(&mut R) + 'static,
{
    fn make_handler(self) -> Box<dyn FnMut(&mut R, Input)> {
        Box::new(move |runtime, _: Input| (self)(runtime))
    }
}

impl<R, F, T> IntoHandler<R, T> for F
where
    R: Runtime,
    F: Fn(&mut R, T) + 'static,
    T: FromExternal,
{
    fn make_handler(self) -> Box<dyn FnMut(&mut R, Input)> {
        Box::new(move |runtime, input: Input| {
            let arg1 = T::from_external(runtime, input);
            (self)(runtime, arg1)
        })
    }
}

/// Implemented for handlers taking two argument.
impl<R, F, T1, T2> IntoHandler<R, (T1, T2)> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2) + 'static,
    T1: FromExternal,
    T2: FromExternal,
{
    fn make_handler(self) -> Box<dyn FnMut(&mut R, Input)> {
        Box::new(move |runtime, input| {
            let arg1 = T1::from_external(runtime, input.clone());
            let arg2 = T2::from_external(runtime, input);
            (self)(runtime, arg1, arg2)
        })
    }
}

pub struct Application<'a, R>
where
    R: Runtime,
{
    runtime: &'a mut R,
    transitions: Vec<Box<dyn FnMut(&mut R, Input)>>,
}

impl<'a, R> Application<'a, R>
where
    R: Runtime,
{
    pub fn new(runtime: &'a mut R) -> Self {
        Self {
            runtime,
            transitions: Vec::default(),
        }
    }

    pub fn register<F, Marker>(&mut self, transition: F) -> &mut Self
    where
        F: IntoHandler<R, Marker> + 'static,
    {
        let fct = transition.make_handler();

        self.transitions.push(fct);
        self
    }

    pub fn run(&mut self) {
        let mut is_running = true;
        while is_running {
            let input = self.runtime.next_input();
            match input {
                None => is_running = false,
                Some(input) => {
                    let _ = self
                        .transitions
                        .iter_mut()
                        .for_each(|transition| transition(self.runtime, input.clone()));
                }
            }
        }
    }
}
