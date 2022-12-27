---
main file: state.rs
supporting files: 
cosmwasm topics:
rust topics:
---

# Overview
> "I'm sorry sir, but you aren't authorized to push that particular button."

# Adding the Minter
In the `Mint` implementation of the The `cw721-base` contract makes an assumption that only one entity is allowed to actually mint NFT tokens. This entity (usually a contract) is set at the time the NFT contract is instantiated. The contract needs to keep track of the minter for later validation.

# Exercise
Since the `minter` is a contract, this means we'll be making use of the `Addr` struct from the `cosmwasm-std` library. 

1. With `use` include the `Addr` struct from `cosmwasm-std`. 
2. Above `token_count` add a new member called `minter` of type `Addr`.

# Starter
```rust
// add the library call here.
pub struct Cw721Contract
{
    pub // add the minter member here.
    pub token_count: u64,
}
```

# Answer
```rust
use cosmwasm_std::Addr
pub struct Cw721Contract
{
    pub minter: Addr,
    pub token_count: u64,
}
```