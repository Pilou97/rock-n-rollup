# Reading custom transfer message

An internal input message can also be a `Transfer` message with a michelson payload.
Let's say you want to receive some bytes ticket, you can add the follow:

```rust
fn transfer<R: Runtime>(rt: &mut R, msg: Internal<Transfer<Ticket<MichelsonBytes>>>) {
    let transfer = msg.payload();
    let ticket = transfer.payload();
    let destination = transfer.destination();
    let source = transfer.source();
    let sender = transfer.sender();
}
```
