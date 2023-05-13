# Reading custom transfer message

An internal input message can also be a `Transfer` message with a michelson payload.
Let's say you want to receive some bytes ticket, you can add the follow:

```rust, noplayground
use rock_n_rollup::core::Runtime;
use rock_n_rollup::services::internal::*;
use rock_n_rollup::core::michelson::*;
use rock_n_rollup::core::michelson::ticket::*;

fn transfer<R: Runtime>(rt: &mut R, msg: Internal<Transfer<Ticket<MichelsonBytes>>>) {
    let transfer = msg.payload();
    let ticket = transfer.payload();
    let destination = transfer.destination();
    let source = transfer.source();
    let sender = transfer.sender();
}
# fn main(){}
```

Your transition will be executed when the payload is a transfer of byte tickets:

```rust, noplayground
use rock_n_rollup::core::{Runtime, Application};
# use rock_n_rollup::services::internal::*;
# use rock_n_rollup::core::michelson::*;
# use rock_n_rollup::core::michelson::ticket::*;

# fn transfer<R: Runtime>(rt: &mut R, msg: Internal<Transfer<Ticket<MichelsonBytes>>>) {
#     let transfer = msg.payload();
#     let ticket = transfer.payload();
#     let destination = transfer.destination();
#     let source = transfer.source();
#     let sender = transfer.sender();
# }

#[rock_n_rollup::main]
pub fn kernel_entry<R: Runtime>(application: &mut Application<R>) {
    application
        .register(transfer)
        .run();
}
# fn main(){}
```
