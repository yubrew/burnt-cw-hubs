# Bunrt Labs - smart contracts

This is the official repo of Burnt Labs suite of smart contracts built on Cosmos with CosmWasm.
These contracts are in constant development and most recent QA approved changes would be at the tip of the testnet branch. More stable versions of the contract are in the main repository. 
The contracts are written in Rust as is the de-facto language for CosmWasm, besides most of our engineers write Rust.

## Setting up for local development
First, [install rust](https://rustup.rs/)

Now you need the wasm toolchaion for Rust
Follow this commands
 ```bash
 rustup update stable
 ```
 and the toolchain
 ```bash
 rustup target add wasm32-unknown-unknown --toolchain nightly
 ```
 and you should be all set. Run the unit-test
 ```bash
 cargo unit-test
 ```
 that should all pass.
 
 **NB:** 
 Should you encounter any issues, please confirm you have the wasm toolchain properly installed.

If you're only interested in getting the contract deployed onn your local chain, you need to first, compile the contract 

```bash
RUSTFLAGS='-C link-arg=-s' cargo wasm
```
to create the optimized *.wasm build in the target/wasm32-unknown-unknown/releases folder.

Now, deploy to your node

```bash
[wasmd] tx wasm store [optimized *.wasm contract] --from [wallet] --node [rpc] --chain-id [chain-id] [flags]
```
which should store the contract to the node and you can instantiate your contract. Refer to each contract docs for instantiate msgs

**NB:**
If you don't want to build the contract from scratch, simply download the latest release and delpoy those

## Going Further

[Developing](./Developing.md) to explain
more on how to run tests and develop code. Or go through the
[online tutorial](https://docs.cosmwasm.com/) to get a better feel
of how to develop.

[Publishing](./Publishing.md) contains useful information on how to publish your contract
to the world, once you are ready to deploy it on a running blockchain. And
[Importing](./Importing.md) contains information about pulling in other contracts or crates
that have been published.
