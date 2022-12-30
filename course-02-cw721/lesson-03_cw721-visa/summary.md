---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
In this lesson, we started down the road of writing our own CW721 based NFT contract, called `cw721-visa`. We did some of the usual things for a CosmWasm contract, such as writing the `InstantiateMsg` struct, and then expanded on the notion of NFT metadata.

In general, we learned about:
- NFT metadata, the OpenSea standard, and the `trait` and `attribute` schema.
- Making practical use of the Rust `Default` trait.
- A slightly deeper look into how `Cw721Contract` is setup and used.

From the `cosmwasm` libraries, we touched on:
- More on the `Addr` object.
- Why `cw_storage_plus` is an upgrade on the older `cosmwasm_storage` package.
- A little about the often seen `cw_serde` procedural macro.

## Next Up
In preparation for updates to the Jump Ring portal contract, we're going to take a short side trip to look at the inner workings of `Cw721ReceiveMsg`. 

<!-- This should be the contract file -->
# Answer
```rust

```