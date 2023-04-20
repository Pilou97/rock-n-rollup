# Installing Rust

If you don't have Rust yet, we recommend you use `rustup` to manage your Rust installation. The official rust guide has a wonderful section on getting started.

Rock-N-Rollup currently has a minimum supported Rust version of 1.66. Running `rustup default 1.66` will ensure you have the correct version of Rust. As such, this guide assumes you are running Rust 1.66.

To develop Smart Rollups on tezos, you will also need to compile your Rust code to wasm. To compile to wasm just add the `wasm32-unknown-unknown` as a new target: `rustup target add wasm32-unknown-unknown`.

# Hello kernel!

Start by creating a new library based Cargo project and changing into the new directory:

```bash
cargo new hello-kernel --lib
cd hello-kernel
```

Add a `lib` section to your Cargo.toml:

```
[lib]
crate-type = ["rlib", "cdylib"]
```

Add `rock-n-rollup` as a dependency of your project by adding the following to your `Cargo.toml` file.

```
[dependencies]
rock-n-rollup = "0.0.1"
```

Transition functions accept zero or more parameters. These parameters can be extracted from an input (see `FromInput` trait) and returns void.

Replace the contents of `src/lib.rs` with the following:

```rust
use rock_n_rollup::core::{Application, Runtime};


fn hello<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello kernel!");
}
```

Next, create a `main` function, that accept an `Application` as parameters. Use `App.register` to add a transition to your application. Finnaly the app is started by calling `run` on it.

```rust
#[rock_n_rollup::main]
pub fn main<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
```

That's it! It should compile with `cargo build --release --target wasm32-unknown-unknown`
