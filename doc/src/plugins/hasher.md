# Hasher

The `Hasher` plugin gives you access to blake2b hashing algorithm with the following function:

- `hash` to hash any data with Blake2b 256 bits
- `hash_512` to hash any data with Blake2b 512 bits

## How to use the Hasher plugin

Let's say you have a `transition`. If you want to use the hasher you just add to add the Hasher trait to the Runtime constraint:

```rust
use rock_n_rollup::plugins::hasher::Hasher;

fn transition<R: Hasher>(rt: &mut R) {
    let data = b"Hello world";
    let hash = rt.hash(data);
    let string: String = hash.to_string();
}
# fn main(){}
```
