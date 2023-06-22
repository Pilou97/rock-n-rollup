# Installing Rust

If you don't have Rust yet, we recommend you use `rustup` to manage your Rust installation. The official rust guide has a wonderful section on getting started.

Rock-N-Rollup currently has a minimum supported Rust version of 1.66. Running `rustup default 1.66` will ensure you have the correct version of Rust. As such, this guide assumes you are running Rust 1.66.

To develop smart rollups on Tezos, you will also need to compile your Rust code to Wasm. To compile to Wasm just add the `wasm32-unknown-unknown` as a new target: `rustup target add wasm32-unknown-unknown`.

# Hello kernel!

Start by creating a new library based Cargo project and changing into the new directory:

```bash
cargo new hello-kernel --lib
cd hello-kernel
```

Add a `[lib]` section to your `Cargo.toml`:

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

Add `rock-n-rollup` as a dependency of your project in `Cargo.toml` file.

```toml
[dependencies]
rock-n-rollup = "0.0.5"
```

Transition functions accept zero or more parameters. These parameters can be extracted from an input (see `FromInput` trait) and returns void.

Now let's start the kernel by replace the contents of `src/lib.rs` with the following:

```rust,noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::core::Runtime;

fn hello<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello kernel!");
}
# fn main(){}
```

Next, create a `kernel_entry` function, that accept an `Application` as parameter. Use `application.register` to add a transition to your application. Finally, the application is started by calling `run` on it.

```rust,noplayground
# extern crate rock_n_rollup;
use rock_n_rollup::core::Application;
use rock_n_rollup::core::Runtime;

fn hello<R: Runtime>(rt: &mut R) {
    rt.write_debug("Hello kernel!");
}

#[rock_n_rollup::main]
pub fn kernel_entry<R: Runtime>(application: &mut Application<R>) {
    application.register(hello).run();
}
# fn main(){}
```

That's it! It should compile with `cargo build --release --target wasm32-unknown-unknown`
