---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
In this lesson, we learned about the CW721 specification that details the requirements for making an NFT contract on the Cosmos blockchain and the `cw721-base` contract that can be used as a template for writing specific NFT contracts.

More specifically, we created the basic form of the `Cw721Contract` struct. It is used by the `cw721-base` contract to keep track of a variety of key fields:
- The contract info of its name and symbol.
- Who the primary minter of NFT tokens is allowed to be.
- How many tokens the contract is tracking.
- Who are valid operators, given permission to send or transfer some or all of the NFTs.
- The list of the tokens and their respective owners.

All of the functionality of the `cw721-base` contract revolve around this structure, in particular the `tokens` and `operators` fields.

From the `cosmwasm` libraries, we touched on:
- The `cw-storage-plus` package, which improves upon the old `Storage` package. In particular, we worked with:
  - `Item`, a useful way to store simple data. 
  - `Map` and its bigger cousin, `IndexedMap` to store and retrieve indexed data.
- The `Expiration` enum from the `cw-utils` package. 

## Next Up

<!-- This should be the contract file -->
# Answer
```rust

```