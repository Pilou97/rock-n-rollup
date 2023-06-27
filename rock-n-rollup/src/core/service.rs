use super::{CustomRuntime, RawInput};
//use tezos_smart_rollup_host::runtime::Runtime;

#[derive(Clone)]
pub struct Input<P> {
    pub level: u32,
    pub id: u32,
    pub payload: P,
}

pub trait FromInput<P, S>
where
    Self: Sized,
{
    fn from_input<R: CustomRuntime>(
        runtime: &mut R,
        input: &Input<P>,
        state: &S,
    ) -> Result<Self, ()>;
}

////////// some types
type TransitionFct<R, P, S> = dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>;

type GuardFct<R, P> = dyn FnMut(&mut R, &Input<P>) -> bool;

pub trait IntoTransition<R, P, S, T>
where
    R: CustomRuntime,
{
    fn into_transition(self) -> Box<TransitionFct<R, P, S>>;
}

// Macro to implements IntoTransition for any function of n parameter
macro_rules! tuple_from_req {
    ($struct_name:ident; $($generic_param:ident),*) => {
        #[allow(non_snake_case)]
        pub struct $struct_name<$($generic_param),*> {
            $( $generic_param: $generic_param ),*
        }

        impl<P, S, $($generic_param),*> FromInput<P, S> for $struct_name<$($generic_param),*>
        where
            $($generic_param: FromInput<P, S>),*,
            P: Clone,
        {
            fn from_input<R: CustomRuntime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
                $(
                    #[allow(non_snake_case)]
                    let $generic_param = match <$generic_param>::from_input(runtime, input, state) {
                        Ok(t) => t,
                        Err(_) => return Err(()),
                    };
                )*

                Ok($struct_name { $($generic_param),* })
            }
        }

        impl<R, P, Fct, S, $($generic_param),*> IntoTransition<R, P, S, $struct_name<$($generic_param),*>> for Fct
            where
                R: CustomRuntime,
                Fct: Fn(&mut R, $($generic_param),*) + 'static,
                $($generic_param: FromInput<P, S>),*,
                P: Clone,
        {
            fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
                Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
                    let tuple = match $struct_name::from_input(runtime, input, state) {
                        Ok(t) => t,
                        Err(_) => return Err(()),
                    };
                    (self)(runtime, $(tuple.$generic_param),*);
                    Ok(())
                })
            }
        }
    };
}

impl<R, P, F, S> IntoTransition<R, P, S, ()> for F
where
    R: CustomRuntime,
    F: Fn(&mut R) + 'static,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        println!("Into transition");
        Box::new(move |runtime: &mut R, _: &Input<P>, _: &S| {
            (self)(runtime);
            Ok(())
        })
    }
}

tuple_from_req!(Tuple1; A);
tuple_from_req!(Tuple2; A, B);
tuple_from_req!(Tuple3; A, B, C);
tuple_from_req!(Tuple4; A, B, C, D);
tuple_from_req!(Tuple5; A, B, C, D, E);
tuple_from_req!(Tuple6; A, B, C, D, E, F);
tuple_from_req!(Tuple7; A, B, C, D, E, F, G);
tuple_from_req!(Tuple8; A, B, C, D, E, F, G, H);
tuple_from_req!(Tuple9; A, B, C, D, E, F, G, H, I);

///// FromInput implementation
pub trait FromRawInput
where
    Self: Sized,
{
    fn from_raw_input<R: CustomRuntime>(runtime: &mut R, input: &RawInput) -> Result<Self, ()>;
}

impl<S> FromInput<Vec<u8>, S> for () {
    fn from_input<R: CustomRuntime>(_: &mut R, _: &Input<Vec<u8>>, _: &S) -> Result<Self, ()> {
        Ok(())
    }
}

impl<P, S> FromInput<P, S> for Input<P>
where
    P: Clone,
{
    fn from_input<R: CustomRuntime>(_: &mut R, input: &Input<P>, _: &S) -> Result<Self, ()> {
        Ok(input.clone())
    }
}

impl<P, S> FromInput<P, S> for P
where
    P: Clone,
{
    fn from_input<R: CustomRuntime>(_: &mut R, input: &Input<P>, _: &S) -> Result<Self, ()> {
        Ok(input.payload.clone())
    }
}

impl FromRawInput for Vec<u8> {
    fn from_raw_input<R: CustomRuntime>(_: &mut R, input: &RawInput) -> Result<Self, ()> {
        Ok(input.payload.clone())
    }
}

impl FromRawInput for () {
    fn from_raw_input<R: CustomRuntime>(_: &mut R, _: &RawInput) -> Result<Self, ()> {
        Ok(())
    }
}

////// Service

pub struct Service<R, P, S>
where
    P: FromRawInput,
{
    guards: Vec<Box<GuardFct<R, P>>>,
    transitions: Vec<Box<TransitionFct<R, P, S>>>,
    state: S,
}

impl<R, P, S> Service<R, P, S>
where
    P: FromRawInput,
{
    pub fn new(state: S) -> Self {
        Self {
            guards: Default::default(),
            transitions: Default::default(),
            state,
        }
    }
}

pub trait Runnable<R>
where
    R: CustomRuntime,
{
    fn run(&mut self, runtime: &mut R, input: RawInput);
}

impl<R, P, S> Runnable<R> for Service<R, P, S>
where
    R: CustomRuntime,
    P: FromRawInput,
{
    fn run(&mut self, runtime: &mut R, input: RawInput) {
        println!("run");

        let payload = match P::from_raw_input(runtime, &input) {
            Ok(payload) => payload,
            Err(_) => todo!("handle this error"),
        };
        println!("payload is present");

        // Get the raw input
        let input = Input {
            level: input.level,
            id: input.id,
            payload,
        };

        // Get the "state"
        let state = &self.state;

        // Run the guards
        let accepted = self.guards.iter_mut().all(|guard| guard(runtime, &input));

        match accepted {
            false => {
                println!("hmmmm");
                // Do nothing on this message
            }
            true => {
                println!("it's accepted");
                // Now we can execute every transitions
                println!("transitions: {}", self.transitions.len());

                for transition in self.transitions.iter_mut() {
                    println!("transition");
                    let _ = transition(runtime, &input, state);
                }
            }
        }
    }
}

impl<R, P, S> Service<R, P, S>
where
    R: CustomRuntime + 'static,
    P: FromRawInput + 'static,
{
    /// Add a guard to the service
    ///
    /// It acts as a service, if the function returns true the message will be processed
    /// otherwise the message will be ignore
    pub fn add_guard(&mut self, guard: fn(&mut R, &Input<P>) -> bool) -> &mut Self {
        let boxed = Box::new(guard);
        self.guards.push(boxed);
        self
    }

    /// Add a transition to the service
    ///
    /// A transition can be any function of one or several parameters
    /// The transition function should take a runtime as first parameter and then other parameters should implement FromInput
    pub fn register<F, Marker>(&mut self, transition: F) -> &mut Self
    where
        F: IntoTransition<R, P, S, Marker> + 'static,
    {
        let fct = transition.into_transition();
        println!("register");
        println!("registered: {}", self.transitions.len());
        self.transitions.push(fct);
        self
    }
}

pub trait IntoService<R, P, S>
where
    R: CustomRuntime,
    P: FromRawInput,
{
    fn into_service(self) -> Service<R, P, S>;
}

impl<R, P, S> IntoService<R, P, S> for Service<R, P, S>
where
    R: CustomRuntime,
    P: FromRawInput,
{
    fn into_service(self) -> Service<R, P, S> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{runtime::MockRuntime, Application, CustomRuntime};
    //use tezos_smart_rollup_host::runtime::Runtime;

    use super::{FromInput, IntoService, Service};

    struct Test {
        inner: String,
    }

    impl FromInput<Vec<u8>, String> for Test {
        fn from_input<R: CustomRuntime>(
            _: &mut R,
            _: &super::Input<Vec<u8>>,
            state: &String,
        ) -> Result<Self, ()> {
            Ok(Self {
                inner: state.clone(),
            })
        }
    }

    fn transition_0<R: CustomRuntime>(rt: &mut R) {
        rt.write_debug("Hello world 0");
    }

    fn transition_1<R: CustomRuntime>(rt: &mut R, t: Test) {
        rt.write_debug(&t.inner);
    }

    fn transition_2<R: CustomRuntime>(rt: &mut R, _: (), _: ()) {
        rt.write_debug("Hello world 2");
    }

    fn transition_3<R: CustomRuntime>(rt: &mut R, _: (), _: (), _: ()) {
        rt.write_debug("Hello world 3");
    }

    fn transition_4<R: CustomRuntime>(rt: &mut R, _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 4");
    }

    fn transition_5<R: CustomRuntime>(rt: &mut R, _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 5");
    }

    fn transition_6<R: CustomRuntime>(rt: &mut R, _: (), _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 6");
    }

    fn transition_7<R: CustomRuntime>(rt: &mut R, _: (), _: (), _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 7");
    }

    fn transition_8<R: CustomRuntime>(
        rt: &mut R,
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
    ) {
        rt.write_debug("Hello world 8");
    }

    fn transition_9<R: CustomRuntime>(
        rt: &mut R,
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
        _: (),
    ) {
        rt.write_debug("Hello world 9");
    }

    #[test]
    fn test() {
        let mut runtime = MockRuntime::default();
        runtime.add_input(Vec::default());
        let mut service = Service::<_, Vec<u8>, String>::new("Hello world 1".to_string());

        service
            .add_guard(|_runtime, _input| true)
            .register(transition_0)
            .register(transition_1)
            .register(transition_2)
            .register(transition_3)
            .register(transition_4)
            .register(transition_5)
            .register(transition_6)
            .register(transition_7)
            .register(transition_8)
            .register(transition_9);

        let () = Application::new(&mut runtime).service(service).run();

        assert_eq!(
            runtime.stdout(),
            vec![
                "Hello world 0",
                "Hello world 1",
                "Hello world 2",
                "Hello world 3",
                "Hello world 4",
                "Hello world 5",
                "Hello world 6",
                "Hello world 7",
                "Hello world 8",
                "Hello world 9",
            ]
        )
    }

    struct CustomService {
        _data: String,
    }

    fn custom_transition<R: CustomRuntime>(_: &mut R) {}

    impl<R> IntoService<R, Vec<u8>, CustomService> for CustomService
    where
        R: CustomRuntime,
    {
        fn into_service(self) -> Service<R, Vec<u8>, Self> {
            let mut service = Service::<R, Vec<u8>, Self>::new(self);
            service.register(custom_transition);
            service
        }
    }

    #[test]
    fn test_2() {
        let mut runtime = MockRuntime::default();
        let mut application = Application::new(&mut runtime);

        let service = CustomService {
            _data: "some data".to_string(),
        };

        application.service(service).run();
    }
}
