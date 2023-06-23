use rock_n_rollup::{
    core::{Application, Runtime},
    plugins::database::{Database, Json},
};

pub struct Counter {
    pub(crate) counter: i64,
}

impl Counter {
    fn increment(self) -> Counter {
        Counter {
            counter: self.counter + 1,
        }
    }

    fn _decrement(self) -> Counter {
        Counter {
            counter: self.counter - 1,
        }
    }
}

pub fn counter<R: Database<Json>>(rt: &mut R) {
    let counter = Counter { counter: 1 };
    let counter_incr = Counter::increment(counter);
    let _ = rt.save("/counter_incr", &counter_incr.counter);
    let counter_read = rt.get::<i64>("/counter_incr").unwrap().unwrap();
    assert_eq!(counter_incr.counter, counter_read);
}

#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(counter).run();
}
