# Installer

You may want to upgrade your kernel.
The `Installer` plugin expose a function to install properly a new kernel.

```rust
fn transition<R: Installer>(rt: &mut R) {
    let kernel: Vec<u8> = ...;
    let result: Result<(), ()> = rt.install(&kernel);
}
```
