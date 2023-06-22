# Database

The `Database` plugin gives you an easier way to read and write data to the durable state.

You only need to use the database to derive `Serialize` and `Deserialize` on your custom types, and you ready to go.

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::database::{Database, Json};

fn transition<R: Database<Json>>(rt: &mut R) {
    let greetings = "Hello world!".to_string();

    let _ = rt.save("/greet", &greetings);
    let greetings = rt.get::<String>("/greet");
}
# fn main(){}
```

## Backends

Rock-N-Rollup gives you 2 backends to handle the serialization and deserialization of your data:

The JSON one, useful when you want to access this data directly from the browser:

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::database::{Database, Json};

fn transition<R: Database<Json>>(rt: &mut R) {
    let greetings = "Hello world!".to_string();

    let _ = rt.save("/greet", &greetings);
    let greetings = rt.get::<String>("/greet");
}
# fn main(){}
```

The bincode one, which is faster and consume less ticks:

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::database::{Database, Bincode};

fn transition<R: Database<Bincode>>(rt: &mut R) {
    let greetings = "Hello world!".to_string();

    let _ = rt.save("/greet", &greetings);
    let greetings = rt.get::<String>("/greet");
}
# fn main(){}
```

## How to read the data from the browser?

If you want to read the data from the browser, we highly recommend to use the JSON backend.

Then let's say you have saved the data `"Hello world"` under the path `"/state"`. Then you can query this endpoint:

```bash
curl "https://rollup.address/global/block/head/durable/wasm_2_0_0/value?key=/state"
```

It should returns you an array of bytes.

The first 4 bytes represent the size of the data.
The remaining bytes represent the JSON. Then you can deserialize these bytes into a string, and then you can use `JSON.parse` onto this string.
