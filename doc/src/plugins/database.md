# Database

The `Database` plugin gives you an easier way to read and to write data to the durable state.

The only thing things you need to use the database is to derive `Serialize` and `Deserialize` on your custom types, and you ready to go.

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
