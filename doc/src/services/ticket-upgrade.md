# TicketUpgrade

A simple strategy to upgrade your smart rollup is the "byte ticket strategy".

The idea is simple:

- you split your kernel into chunks
- you send the root hash to a smart contract
- this smart contract sends a byte ticket representing the root hash to your rollup
- your rollup proceed to the upgrade

## Requirement

- Your rollup should have the type `byte ticket`
- Your rollup should be deployed with the `TicketUpgrade` service
- Deploying a smart contract on L1 that acts as a proxy

## Deploy a smart contract

TODO: provide a minimal example:

Here is the minimal specification of the smart contract:

- one entrypoint that accepts bytes
- check the signature of the sender (you can also check the authencity of the bytes with a multisig or what you want)
- mint a byte ticket, the content of the bytes, is the input of the entrypoint
- send the byte ticket to your rollup address

## How to install the service

Let's say you have your application, if you want to add the service the only things you have to do is the following:

```rust
fn main<R: Runtime>(application: &mut Application<Runtime>) {
    application
        .service(TicketUpgrade::new("KT1...")) // Put the address of your L1 contract
        .transition(...)
        .transition(...)
        .transition(...)
        .transition(...)
        .run()
}
```

Then when your kernel will receive a root hash from this contract it will proceed to the installation of your new kernel.

## Split your kernel

As you did to originate your kernel, you will have to split your new kernel with the [tezos-smart-rollup-installer](https://crates.io/crates/tezos-smart-rollup-installer).

```bash
$ smart-rollup-installer get-reveal-installer --upgrade-to my_kernel.wasm --output installer.hex --preimages-dir wasm_2_0_0
```

Place the generated chunks in the good folder.

Then you need to retrieve the root hash:

```bash
$ cat installer.hex | tail -c 33
```

You can send the resulting output directly to your smart contract and your kernel will be upgraded!!
