---
main file: msg.rs
supporting files: 
cosmwasm topics:
rust topics:
---

# Overview
> The Administrators of Planetary Experience ("Apes") *claim* to be neutral when it comes to overseeing access to the Exodite's more popular eco-worlds. 

This lesson is about building our own NFT contract, using `cw721-base` as, well, the basis for it. The `cw721-base` provides us with most of what a new NFT contract needs to support the CW721 specification, such as the various approval and revoke functions. As noted earlier, the act of minting, ubiquitous to NFTs, is *not* a part of the standard, though the `cw721-base` contract does provide a `mint` function for basic functionality.

For this lesson, we're going to leverage the `cw721-base` template by making minor changes to how minting is expected to work. This should be enough to give you ideas on how to expand on your own ideas for making a CosmWasm NFT contact.

# InstantiateMsg
In the `cw721-base` contract, the `InstantiateMsg` had three fields, `name`, `symbol`, and `minter`. We'll be replacing the `minter` field with an `apes` vector field and adding a new field for the Jump Ring address (which we'll expand on in a later course).

# Exercise
This is simple warmup exercise; since you've already done this a few times, we're asking you to fill in the entire `InstantiateMsg` struct.

1. Create the `InstantiateMsg` struct. Make it (and all its fields) public.
2. Add the four public fields in alphabetical order: `apes`, `jump_ring`, `name`, and `symbol`. 
    - `apes` is an `Addr` vector.
    - `jump_ring` is of type `Addr`.
    - `name` and `symbol` are both of type `String`.

# Starter
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
// add the full InstantiateMsg struct here.

```

# Answer
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub apes: Vec<Addr>,
    pub jump_ring: Addr,
    pub name: String,
    pub symbol: String,
}
```