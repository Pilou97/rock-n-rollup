use super::{RawInput, Runtime};

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
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()>;
}

////////// some types
type TransitionFct<R, P, S> = dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>;

type GuardFct<R, P> = dyn FnMut(&mut R, &Input<P>) -> bool;

pub trait IntoTransition<R, P, S, T>
where
    R: Runtime,
{
    fn into_transition(self) -> Box<TransitionFct<R, P, S>>;
}

///// 0 argument

impl<R, P, F, S> IntoTransition<R, P, S, ()> for F
where
    R: Runtime,
    F: Fn(&mut R) + 'static,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, _: &Input<P>, _: &S| {
            (self)(runtime);
            Ok(())
        })
    }
}

//// One argument

pub struct Tuple1<T1> {
    pub t1: T1,
}

impl<P, S, T1> FromInput<P, S> for Tuple1<T1>
where
    T1: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple1 { t1 })
    }
}

impl<R, P, F, S, T1> IntoTransition<R, P, S, Tuple1<T1>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1) + 'static,
    T1: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let single = match Tuple1::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(runtime, single.t1);
            Ok(())
        })
    }
}

////// Two arguments

pub struct Tuple2<T1, T2> {
    pub t1: T1,
    pub t2: T2,
}

impl<P, S, T1, T2> FromInput<P, S> for Tuple2<T1, T2>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple2 { t1, t2 })
    }
}

impl<R, P, F, S, T1, T2> IntoTransition<R, P, S, Tuple2<T1, T2>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let pair = match Tuple2::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(runtime, pair.t1, pair.t2);
            Ok(())
        })
    }
}

////// Three arguments

pub struct Tuple3<T1, T2, T3> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
}

impl<P, S, T1, T2, T3> FromInput<P, S> for Tuple3<T1, T2, T3>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple3 { t1, t2, t3 })
    }
}

impl<R, P, F, S, T1, T2, T3> IntoTransition<R, P, S, Tuple3<T1, T2, T3>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let pair = match Tuple3::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(runtime, pair.t1, pair.t2, pair.t3);
            Ok(())
        })
    }
}

/// Four arguments
pub struct Tuple4<T1, T2, T3, T4> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
}

impl<P, S, T1, T2, T3, T4> FromInput<P, S> for Tuple4<T1, T2, T3, T4>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple4 { t1, t2, t3, t4 })
    }
}

impl<R, P, F, S, T1, T2, T3, T4> IntoTransition<R, P, S, Tuple4<T1, T2, T3, T4>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple4::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4);
            Ok(())
        })
    }
}

/// Five arguments
pub struct Tuple5<T1, T2, T3, T4, T5> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
    pub t5: T5,
}

impl<P, S, T1, T2, T3, T4, T5> FromInput<P, S> for Tuple5<T1, T2, T3, T4, T5>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t5 = match T5::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple5 { t1, t2, t3, t4, t5 })
    }
}

impl<R, P, F, S, T1, T2, T3, T4, T5> IntoTransition<R, P, S, Tuple5<T1, T2, T3, T4, T5>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4, T5) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple5::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4, tuple.t5);
            Ok(())
        })
    }
}

/// Six arguments
pub struct Tuple6<T1, T2, T3, T4, T5, T6> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
    pub t5: T5,
    pub t6: T6,
}

impl<P, S, T1, T2, T3, T4, T5, T6> FromInput<P, S> for Tuple6<T1, T2, T3, T4, T5, T6>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t5 = match T5::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t6 = match T6::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple6 {
            t1,
            t2,
            t3,
            t4,
            t5,
            t6,
        })
    }
}

impl<R, P, F, S, T1, T2, T3, T4, T5, T6> IntoTransition<R, P, S, Tuple6<T1, T2, T3, T4, T5, T6>>
    for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4, T5, T6) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple6::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(
                runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4, tuple.t5, tuple.t6,
            );
            Ok(())
        })
    }
}

/// Seven arguments
pub struct Tuple7<T1, T2, T3, T4, T5, T6, T7> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
    pub t5: T5,
    pub t6: T6,
    pub t7: T7,
}

impl<P, S, T1, T2, T3, T4, T5, T6, T7> FromInput<P, S> for Tuple7<T1, T2, T3, T4, T5, T6, T7>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t5 = match T5::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t6 = match T6::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t7 = match T7::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple7 {
            t1,
            t2,
            t3,
            t4,
            t5,
            t6,
            t7,
        })
    }
}

impl<R, P, F, S, T1, T2, T3, T4, T5, T6, T7>
    IntoTransition<R, P, S, Tuple7<T1, T2, T3, T4, T5, T6, T7>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4, T5, T6, T7) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple7::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(
                runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4, tuple.t5, tuple.t6, tuple.t7,
            );
            Ok(())
        })
    }
}

/// Eight arguments
pub struct Tuple8<T1, T2, T3, T4, T5, T6, T7, T8> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
    pub t5: T5,
    pub t6: T6,
    pub t7: T7,
    pub t8: T8,
}

impl<P, S, T1, T2, T3, T4, T5, T6, T7, T8> FromInput<P, S>
    for Tuple8<T1, T2, T3, T4, T5, T6, T7, T8>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    T8: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t5 = match T5::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t6 = match T6::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t7 = match T7::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t8 = match T8::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple8 {
            t1,
            t2,
            t3,
            t4,
            t5,
            t6,
            t7,
            t8,
        })
    }
}

impl<R, P, F, S, T1, T2, T3, T4, T5, T6, T7, T8>
    IntoTransition<R, P, S, Tuple8<T1, T2, T3, T4, T5, T6, T7, T8>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4, T5, T6, T7, T8) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    T8: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple8::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(
                runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4, tuple.t5, tuple.t6, tuple.t7,
                tuple.t8,
            );
            Ok(())
        })
    }
}

/// Nine arguments
pub struct Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    pub t1: T1,
    pub t2: T2,
    pub t3: T3,
    pub t4: T4,
    pub t5: T5,
    pub t6: T6,
    pub t7: T7,
    pub t8: T8,
    pub t9: T9,
}

impl<P, S, T1, T2, T3, T4, T5, T6, T7, T8, T9> FromInput<P, S>
    for Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
where
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    T8: FromInput<P, S>,
    T9: FromInput<P, S>,
    P: Clone,
{
    fn from_input<R: Runtime>(runtime: &mut R, input: &Input<P>, state: &S) -> Result<Self, ()> {
        let t1 = match T1::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t2 = match T2::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t3 = match T3::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t4 = match T4::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t5 = match T5::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t6 = match T6::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t7 = match T7::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t8 = match T8::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        let t9 = match T9::from_input(runtime, input, state) {
            Ok(t) => t,
            Err(_) => return Err(()),
        };
        Ok(Tuple9 {
            t1,
            t2,
            t3,
            t4,
            t5,
            t6,
            t7,
            t8,
            t9,
        })
    }
}

impl<R, P, F, S, T1, T2, T3, T4, T5, T6, T7, T8, T9>
    IntoTransition<R, P, S, Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9>> for F
where
    R: Runtime,
    F: Fn(&mut R, T1, T2, T3, T4, T5, T6, T7, T8, T9) + 'static,
    T1: FromInput<P, S>,
    T2: FromInput<P, S>,
    T3: FromInput<P, S>,
    T4: FromInput<P, S>,
    T5: FromInput<P, S>,
    T6: FromInput<P, S>,
    T7: FromInput<P, S>,
    T8: FromInput<P, S>,
    T9: FromInput<P, S>,
    P: Clone,
{
    fn into_transition(self) -> Box<dyn FnMut(&mut R, &Input<P>, &S) -> Result<(), ()>> {
        Box::new(move |runtime: &mut R, input: &Input<P>, state: &S| {
            let tuple = match Tuple9::from_input(runtime, input, state) {
                Ok(p) => p,
                Err(_) => return Err(()),
            };
            (self)(
                runtime, tuple.t1, tuple.t2, tuple.t3, tuple.t4, tuple.t5, tuple.t6, tuple.t7,
                tuple.t8, tuple.t9,
            );
            Ok(())
        })
    }
}

///// FromInput implementation
pub trait FromRawInput
where
    Self: Sized,
{
    fn from_raw_input<R: Runtime>(runtime: &mut R, input: &RawInput) -> Result<Self, ()>;
}

impl<S> FromInput<Vec<u8>, S> for () {
    fn from_input<R: Runtime>(_: &mut R, _: &Input<Vec<u8>>, _: &S) -> Result<Self, ()> {
        Ok(())
    }
}

impl<P, S> FromInput<P, S> for Input<P>
where
    P: Clone,
{
    fn from_input<R: Runtime>(_: &mut R, input: &Input<P>, _: &S) -> Result<Self, ()> {
        Ok(input.clone())
    }
}

impl<P, S> FromInput<P, S> for P
where
    P: Clone,
{
    fn from_input<R: Runtime>(_: &mut R, input: &Input<P>, _: &S) -> Result<Self, ()> {
        Ok(input.payload.clone())
    }
}

impl FromRawInput for Vec<u8> {
    fn from_raw_input<R: Runtime>(_: &mut R, input: &RawInput) -> Result<Self, ()> {
        Ok(input.payload.clone())
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
            state: state,
        }
    }
}

pub trait Runnable<R>
where
    R: Runtime,
{
    fn run(&mut self, runtime: &mut R, input: RawInput);
}

impl<R, P, S> Runnable<R> for Service<R, P, S>
where
    R: Runtime,
    P: FromRawInput,
{
    fn run(&mut self, runtime: &mut R, input: RawInput) {
        let payload = match P::from_raw_input(runtime, &input) {
            Ok(payload) => payload,
            Err(_) => todo!("handle this error"),
        };

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
                // Do nothing on this message
            }
            true => {
                // Now we can execute every transitions
                for transition in self.transitions.iter_mut() {
                    let _ = transition(runtime, &input, &state);
                }
            }
        }
    }
}

impl<R, P, S> Service<R, P, S>
where
    R: Runtime + 'static,
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
        self.transitions.push(fct);
        self
    }
}

pub trait IntoService<R, P>
where
    R: Runtime,
    P: FromRawInput,
    Self: Sized,
{
    fn into_service(self) -> Service<R, P, Self>;
}

impl<R, S, P> IntoService<R, P> for S
where
    R: Runtime,
    P: FromRawInput,
{
    fn into_service(self) -> Service<R, P, Self> {
        Service::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{runtime::MockRuntime, Application, Runtime};

    use super::Service;

    fn transition_0<R: Runtime>(rt: &mut R) {
        rt.write_debug("Hello world 0");
    }

    fn transition_1<R: Runtime>(rt: &mut R, _: ()) {
        rt.write_debug("Hello world 1");
    }

    fn transition_2<R: Runtime>(rt: &mut R, _: (), _: ()) {
        rt.write_debug("Hello world 2");
    }

    fn transition_3<R: Runtime>(rt: &mut R, _: (), _: (), _: ()) {
        rt.write_debug("Hello world 3");
    }

    fn transition_4<R: Runtime>(rt: &mut R, _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 4");
    }

    fn transition_5<R: Runtime>(rt: &mut R, _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 5");
    }

    fn transition_6<R: Runtime>(rt: &mut R, _: (), _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 6");
    }

    fn transition_7<R: Runtime>(rt: &mut R, _: (), _: (), _: (), _: (), _: (), _: (), _: ()) {
        rt.write_debug("Hello world 7");
    }

    fn transition_8<R: Runtime>(
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

    fn transition_9<R: Runtime>(
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
        let mut service = Service::<MockRuntime, Vec<u8>, ()>::new(());

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

        let () = Application::new(&mut runtime)
            .service::<Vec<u8>>(service)
            .run();

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

    #[test]
    fn test_2() {
        let mut runtime = MockRuntime::default();
        let mut application = Application::new(&mut runtime);

        let service = CustomService {
            _data: "some data".to_string(),
        };

        application.service::<Vec<u8>>(service).run();
    }
}
