# Internal Messages

Smart rollups always inject 4 messages in the inbox:

- `StartOfLevel` that indicates the beginning of the inbox.
- `InfoPerLevel` that gives you the current Tezos level of this execution.
- `EndOfLevel` that indicates the end of the inbox.
- `Transfer` that takes a transfer.

An easy way to execute a transition of this kind of message is to add a parameter to your transition function:

```rust, noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::services::internal::*;
use rock_n_rollup::core::Runtime;

fn start_of_level<R: Runtime>(rt: &mut R, msg: Internal<StartOfLevel>) {
    // Only executed on StartOfLevel message
    // ...
}

fn info_per_level<R: Runtime>(rt: &mut R, msg: Internal<InfoPerLevel>) {
    // Only executed on InfoPerLevel message
    // ...
}

fn end_of_level<R: Runtime>(rt: &mut R, msg: Internal<EndOfLevel>) {
    // Only executed on EndOfLevel
    // ...
}

fn transfer<R: Runtime>(rt: &mut R, msg: Internal<Transfer<Vec<u8>>>){
    // Only excuted on Transfer
    // ...
}
# fn main(){}
```

# External message

Message from users can come from the `add_rollup_message` Tezos operation. This message will be added to the inbox as an external message.

If you want to trigger a transition on a `External` message, you can add a parameter to your transition function as follow:

```rust, noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::services::external::*;
use rock_n_rollup::core::Runtime;

fn transition<R: Runtime>(rt: &mut R, msg: External<Vec<u8>>) {
    // Only executed on external messages where the payload can be parsed as bytes
    // External<Vec<u8>> will match on any messages
    // ...
}
# fn main(){}
```

At this point, you get all the features as the already provided kernel library provided by Tezos code dev.
You can define transitions on any messages.
