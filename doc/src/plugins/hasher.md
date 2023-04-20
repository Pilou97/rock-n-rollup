# Hasher

The `Hasher` plugin gives you access to blake2b hashing algorithm with the following function:

- `hash` to hash any data with Blake2b 256 bits
- `hash_512` to hash any data with Blake2b 512 bits

## How to use the Hasher plugin

Let's say you have a `transition`. If you want to use the hasher you just add to add the Hasher trait to the Runtime constraint:

```rust
fn transition<H: Hasher>(hasher: &mut H) {
    let hash1 = hasher.hash(vec![0x01, 0x02, 0x03, 0x03]);
    let string: String = hash1.to_string();
    assert!(hash1 == hash2);
}
```
