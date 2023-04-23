# Cryptography

The `Crypto` plugin gives you access to cryptography primitive, like public key and signature verification.

# New types

It defines the `PublicKey` and the `Signature` type. These types can be constructed from a String:

```rust
fn my_function() {
    let pkey = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK".to_string();
    let pkey = PublicKey::try_from(pkey).unwrap();

    let signature = "edsigtuU5nUqBniorqTFXFixkG6ZkfvEPrfc9aT9DnMAeims2AX2yjpgYaedXBoKzAGHE3ZXSi1hZz6piZ3itTE7f2F4FoaxXtM".to_string();
    let signature = Signature::try_from(signature).unwrap();
}
```

# How to use the Crypto plugin

Let's say you have a `transition`. If you want to use the crypto plugin you just have to add the Crypto trait to the Runtime constraint:

```rust
fn transition<C: Crypto>(crypto: &mut C) {
    let pkey = "edpkuDMUm7Y53wp4gxeLBXuiAhXZrLn8XB1R83ksvvesH8Lp8bmCfK".to_string();
    let pkey = PublicKey::try_from(pkey).unwrap();

    let signature = "edsigtuU5nUqBniorqTFXFixkG6ZkfvEPrfc9aT9DnMAeims2AX2yjpgYaedXBoKzAGHE3ZXSi1hZz6piZ3itTE7f2F4FoaxXtM".to_string();
    let signature = Signature::try_from(signature).unwrap();

    let data = "hello world".as_byte();
    let is_correct: bool = crypto.verify(&signature, &pkey, data);
}
```