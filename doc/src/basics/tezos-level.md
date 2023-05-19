# Tezos level

You can have access to the Tezos level in your transition

```rust,noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;
use rock_n_rollup::services::level::Level;

fn transition<R: Runtime>(rt: &mut R, level: Level) {
    // do what you want with the level
    let level: &u32 = level.as_ref();
}
# fn main(){}
```
