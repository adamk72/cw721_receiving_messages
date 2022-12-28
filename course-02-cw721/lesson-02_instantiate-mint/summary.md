---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
In this lesson, we took the time to recreate the `instantiate` entry point and the `ExecuteMsg::mint` function from the `cw721-base` contract. The goal was to become familiar with the key components that make up an NFT function and to describe some CosmWasm functions more thoroughly.

In general, we learned about:
- How `cw721-base` contract makes use of the `Default` trait so we can use it as a template for our own contracts.
- How the `minter` is required by the `cw721-base` contract, but is not specifically a CW721 spec requirement.
- The ERC721 metadata schema that is suggested for the metadata extension.

From the `cosmwasm` libraries, we touched on:
- The CW2 spec for defining and updating contract info.
- How the `Item::save` and `Item::load` functions work.
- Details on the `IndexedMap::update` function.


## Next Up

<!-- This should be the contract file -->
# Answer
```rust

```