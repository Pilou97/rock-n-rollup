# Installer

You may want to upgrade your kernel.
The `Installer` plugin exposes a function to install properly a new kernel.

```rust
# extern crate rock_n_rollup;
use rock_n_rollup::plugins::installer::*;

fn transition<R: Installer>(rt: &mut R) {
    let kernel: Vec<u8> = Vec::default(); // let's say you have some bytes
    let result: Result<(), ()> = rt.install(&kernel);
}
# fn main(){}
```
