# Reading custom external messages

If you want to define your custom external message, it's easy:
First let's define a message type:

```rust, noplayground
enum PingPong {
    Ping,
    Pong
}
# fn main(){}
```

The next step is to implement the `FromExternal` trait:

```rust, noplayground
use rock_n_rollup::services::external::*;
# enum PingPong {
#    Ping,
#    Pong
# }

impl FromExternal for PingPong {
    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
        // Notice that the magic byte is not there
        match input.as_slice() {
            [0x00] => Ok(PingPong::Ping),
            [0x01] => Ok(PingPong::Pong),
            _ => Err(())
        }
    }
}
# fn main(){}
```

Then you can use the `PingPong` type in your transition

```rust, noplayground
use rock_n_rollup::core::Runtime;
use rock_n_rollup::services::external::*;

# enum PingPong {
#     Ping,
#     Pong
# }
#
# impl FromExternal for PingPong {
#    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
#        // Notice that the magic byte is not there
#        match input.as_slice() {
#            [0x00] => Ok(PingPong::Ping),
#            [0x01] => Ok(PingPong::Pong),
#            _ => Err(())
#        }
#    }
# }

fn transition<R: Runtime>(rt: &mut R, ping_pong: External<PingPong>) {
    // process your stuff
}
# fn main(){}
```

Your transition will be executed when the payload will be parsed: when there is an external message containing only 0x00 or only 0x01

```rust, noplayground
use rock_n_rollup::core::{Runtime, Application};
# use rock_n_rollup::services::external::*;
#
# enum PingPong {
#     Ping,
#     Pong
# }
#
# impl FromExternal for PingPong {
#    fn from_external(input: Vec<u8>) -> Result<Self, ()> {
#        // Notice that the magic byte is not there
#        match input.as_slice() {
#            [0x00] => Ok(PingPong::Ping),
#            [0x01] => Ok(PingPong::Pong),
#            _ => Err(())
#        }
#    }
# }
#
# fn external_transition<R: Runtime>(rt: &mut R, ping_pong: External<PingPong>) {
#    // process your stuff
# }

#[rock_n_rollup::main]
pub fn kernel_entry<R: Runtime>(application: &mut Application<R>) {
    application
        .register(external_transition)
        .run();
}
# fn main(){}
```
