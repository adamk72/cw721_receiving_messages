---
main file: contract.rs
supporting files: state.rs
cosmwasm topics:
rust topics: impl, Default
---

# Overview
> Long ago, in an effort to protect their planetary cluster, the Exodites tweaked their Jump Rings to prevent unwanted visitors. The technical and economic consequences of the changes are still not fully understood.

# Cw721Contract Review
In the previous lesson, we built up on the `Cw721Contract` struct, which you can see on the `state.rs` tab if you need a refresher. In Rust, you can add code functionality through the `impl` feature, which is how the `cw721-base` contract establishes the base functionality for an NFT contract. Its basic skeleton looks like this:

```rust
impl<'a, T, C, E, Q> Cw721Contract<'a, T, C, E, Q>
{
    pub fn instantiate(..) -> StdResult<Response<C>> {}
    pub fn execute(..) -> Result<Response<C>, ContractError> {}
}
```

# Instantiate
We're going to start this lesson by going over the `InstantiateMsg` and `instantiate` default function for `cw721-base`. In our custom contract, we'll call this message via `Cw721Contract::default().instantiate` which will do the default work for us; we will then make custom changes for our own contract to work.

> ### Default
> Our ability to call `default()` like this come from the `Default` Rust [trait](https://doc.rust-lang.org/std/default/trait.Default.html)<ExternalLink> which is implemented for `Cw721Contract`. This allows developers to set reasonable defaults values when instantiating a struct.

We won't be going over the `execute` function as, it is your typical CosmWasm `match msg {}` function (where all of the details are to support the Cw721 spec). We will go specifically over the `Mint` function, however, which is *not* part of the Cw721 spec.

# Exercise
The first thing we're going to is establish the `InstantiateMsg`. It's a simple struct with three member, each of which are of type `String`.

1. Add a `pub` field called 'name'. This will be the name of the NFT contact when instantiated.
2. Add a `pub` field called 'symbol' &mdash; the symbol to represent the contract, usually three to four characters long.
3. Add a `pub` field called 'minter'. Only a single entity can create new NFTs; this is usually another contract.

Note that the concept of the `minter` is a feature of the `cw721-base` contract and not the Cw721 spec itself. A custom contract could enable different behavior.


# Starter
```rust
use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct InstantiateMsg {
    // name field
    // symbol field 
    // minter field
}
```

# Answer
```rust
use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
}
```