# Inbox index

You can have access to the Tezos level in your transition

```rust,noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;
use rock_n_rollup::services::index::Index;

fn transition<R: Runtime>(rt: &mut R, index: Index) {
    // do what you want with the index
    let index: &u32 = index.as_ref();
}
# fn main(){}
```
