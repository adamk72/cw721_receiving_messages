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
We're going to take a trip to past and make some changes to the Jump Ring contract so we can learn about transferring NFTs between contracts!

<!-- This should be the contract file -->
# Answer
```rust

```