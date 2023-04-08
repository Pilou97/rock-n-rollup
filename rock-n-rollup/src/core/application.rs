use super::{middleware::Middleware, Input, Runtime};

pub trait FromInput
where
    Self: Sized,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: Input) -> Result<Self, ()>;
}

impl FromInput for Input {
    fn from_input<R: Runtime>(_: &mut R, input: Input) -> Result<Self, ()> {
        Ok(input)
    }
}

pub trait IntoTransition<R, T>
where
    R: Runtime,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, Input) -> Result<(), ()>>;
}

impl<F, R> IntoTransition<R, ()> for F
where
    R: Runtime,
    F: Fn(&mut R) + 'static,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, Input) -> Result<(), ()>> {
        Box::new(move |runtime, _: Input| {
            (self)(runtime);
            Ok(())
        })
    }
}

impl<R, F, T> IntoTransition<R, T> for F
where
    R: Runtime,
    F: Fn(&mut R, T) + 'static,
    T: FromInput,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, Input) -> Result<(), ()>> {
        Box::new(move |runtime, input: Input| {
            let arg1 = T::from_input(runtime, input);
            match arg1 {
                Ok(arg1) => {
                    (self)(runtime, arg1);
                    Ok(())
                }
                Err(_) => Err(()),
            }
        })
    }
}

/// Implemented for transition taking two argument.
impl<R, F, T1, T2> IntoTransition<R, (T1, T2)> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2) + 'static,
    T1: FromInput,
    T2: FromInput,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, Input) -> Result<(), ()>> {
        Box::new(move |runtime, input| {
            let arg1 = T1::from_input(runtime, input.clone());
            let arg2 = T2::from_input(runtime, input);
            match (arg1, arg2) {
                (Ok(arg1), Ok(arg2)) => {
                    (self)(runtime, arg1, arg2);
                    Ok(())
                }
                _ => Err(()),
            }
        })
    }
}

pub trait App {
    type R: Runtime + 'static;

    fn register<F, Marker>(&mut self, transition: F) -> &mut Self
    where
        F: IntoTransition<Self::R, Marker> + 'static;

    fn wrap(&mut self, middleware: impl Middleware + 'static) -> &mut Self;

    fn run(&mut self);
}

pub struct Application<'a, R>
where
    R: Runtime,
{
    runtime: &'a mut R,
    transitions: Vec<Box<dyn FnMut(&mut R, Input) -> Result<(), ()>>>,
    middleware: Vec<Box<dyn Middleware>>,
}

impl<'a, R: Runtime + 'static> App for Application<'a, R> {
    type R = R;

    fn register<F, Marker>(&mut self, transition: F) -> &mut Self
    where
        F: IntoTransition<R, Marker> + 'static,
    {
        let fct = transition.into_transition();

        self.transitions.push(fct);
        self
    }

    fn wrap(&mut self, middleware: impl Middleware + 'static) -> &mut Self {
        self.middleware.push(Box::new(middleware));
        self
    }

    fn run(&mut self) {
        let mut is_running = true;
        while is_running {
            let input = self.runtime.next_input();
            match input {
                None => is_running = false,
                Some(input) => {
                    let input = self.middleware.iter().fold(Ok(input), |input, middleware| {
                        input.and_then(|input| middleware.upgrade(input))
                    });

                    match input {
                        Err(_) => {}
                        Ok(input) => {
                            for transition in self.transitions.iter_mut() {
                                match transition(self.runtime, input.clone()) {
                                    Ok(_) => {
                                        break;
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                    }
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
            transitions: Vec::default(),
            middleware: Vec::default(),
        }
    }
}
