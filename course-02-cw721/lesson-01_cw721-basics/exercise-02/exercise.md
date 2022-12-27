---
main file: one file name
supporting files: comma separated list
cosmwasm topics:
rust topics: lib.rs, modules
---

# Overview
> It's a beautiful sight to behold. A physical visa form with all of the right fields filled in properly and all necessary supporting documents stapled to it.

# The `cw721-base` contract
The CosmWasm org has provided the community with a basic contract that implements the CW721 specification and which also shows how `Mint` function might be implemented. For the `cw721-base` contract, it is written as Rust template that we can extend off of, which will be the main part of of <Lesson>. 

## The `Cw721Contract` struct 
The heart of the `cw721-base` contract is the `Cw721Contract` struct. This struct holds all of the parts of the contract that are of interest to us. Of interest to us from a CosmWasm point of view is that this struct contains a lot of elements that come from the `cw-storage-plus` library, a newer and better way for storing data on the blockchain than compared to using the original `Storage` module from `cosmwasm-std` library like we used in the introduction to CosmWasm course.

 We will be building the `Cw721Contract` struct in incremental stages so that you can see it evolve from a rudimentary object to a more useful product to be used as the main entry point for our eventual specialized contract.

# Exercise
Let's ease into this by creating the a basic public structure and one single element, `token_count` which will be a `u64` type. 

1. Add a public struct called `Cw721Contract`. 
2. Add a public member called `token_id` of type `u64`.

Reminder, Rust struct members should terminate with a comma, even if they are just a single member.

# Starter
```rust
pub // Name the struct here
{
  pub // add the member here.
}
```

# Answer
```rust
pub struct Cw721Contract
{
    pub token_count: u64,
}
```