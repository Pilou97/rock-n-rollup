# Database

The `Database` plugin gives you an easier way to read and to write data to the durable state.

The only thing things you need to use the database is to derive `Serialize` and `Deserialize` on your custom types, and you ready to go.

```rust
fn transition<D: Database>(database: &mut D) {
    let greetings = "Hello world!";

    let _ = database.save("/greet", greetings);
    let greetings = database.read::<String>("/greet");
}
```
