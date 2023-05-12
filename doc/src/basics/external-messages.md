# Reading custom external messages

If you want to define your custom external message, it's easy:
First let's define a message type:

```rust
enum PingPong {
    Ping,
    Pong
}
```

The next step is to implement the `FromExternal` trait:

```rust
impl FromExternal for PingPong {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        // Notice that the magic byte is not there
        match input.as_ref() {
            [0x00] => Ok(PingPong::Ping)
            [0x01] => Ok(PingPong::Pong)
            _ => Err(())
        }
    }
}
```

Then you can use the `PingPong` type in your transition

```rust
fn transition<R: Runtime>(&mut rt: R, ping_pong: External<PingPong>) {
    /// process your stuff
}
```
