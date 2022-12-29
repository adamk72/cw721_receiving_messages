---
main file: state.rs
supporting files: addresses.rs, cosmwasm-scheme-lib.rs
cosmwasm topics:
rust topics:
---

# Overview
> The trouble with travel between Exodite eco-worlds is that you need a new visa for each Jump Ring. Guess I'll get in queue... *again*. I hear the Mel'Mirrys Asteroid of Sargas 4 is breathtaking to see.

# CosmWasm Packages 
We're going to a break from coding for a moment and talk about some of the Rust packages that we commonly see use for CosmWasm development. 

## cosmwasm_std
`Addr` you've seen before many times. It is from the ubiquitous `cosmwasm_std` crate. A couple of functions you can use from `Addr` include:
- `Addr::unchecked()` for creating quick and dirty valid addresses for testing purposes.
- `Addr::addr_validate()` which we've talked about for validating real addresses.
- `Addr::addr_humanize()` is new; this takes a binary canonical address and makes it human readable.

## cw_storage_plus
`Item` is from the `cw_storage_plus` crate. We've used it a few times already along with `Map` and `IndexedMap`; it replaces the `cosmwasm_storage` crate after the developers realized some of the limitations of their original design, a credit to them working to make things better for the CosmWasm ecosystem.

## cw_serde
You've seen the `#[cw_serde]` attribute macro a few times in this lesson, but we haven't talked about it. Where the `serde` package is the general use serialization/deserialization library, `cw_serde` is a [procedural macr](https://doc.rust-lang.org/reference/procedural-macros.html)<ExternalLink> from the `cosmwasm-schema` package. It encapsulates a whole host of traits and functions for the (de)serialization process to make it specific for the needs of the Cosmos ecosystem.

# Exercise
In this exercise, we're going to flip the script. We'll give you the code and you fill in the imports to match.

1. Fill in the `use` imports for `cw_serde`, `Addr`, and `Item`.

# Starter
```rust
// use for cw_serde
// use for Addr
// use for Item

#[cw_serde]
pub struct Config {
    pub apes: Vec<Addr>,
    pub jump_ring: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
```

# Answer
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub apes: Vec<Addr>,
    pub jump_ring: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
```