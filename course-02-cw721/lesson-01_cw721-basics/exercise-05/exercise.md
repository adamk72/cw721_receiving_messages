---
main file: state.rs
supporting files: 
cosmwasm topics:
rust topics:
---

# Overview
> 

# The `Item` Struct
We're going to introduce a couple of concepts from from `cw_storage_plus` that are used in the `cw721-base` contract and that you'll likely want to use in your own contracts. 

The first one is `Item`, which looks like this:
```rust
pub struct Item<'a, T> {
    storage_key: &'a [u8],
    data_type: PhantomData<T>  
```
> ### PhantomData
> You're going to see `PhantomData` pop up a lot in our examples. See https://doc.rust-lang.org/std/marker/struct.PhantomData.html#unused-type-parameters for why this is needed. We won't go into details about it in this course; just be aware of it.

`Item` provides easy to use implementations for saving and retrieving singular pieces of data, including `new`, `save`, `load`, `update`, and `query`. Blockchain data needs to be (de)serializable so that it can be written to and read from the blockchain, so this object makes our lives easier to do that. It's very useful for storing basic information.

Notice that the `Item` object requires us to be conscious of the lifetime of our references. So, when writing code using the `Item` type, you need to wrap it appropriately, like so:
```rust
use cw_storage_plus::Item;
struct RacesContract<'a> {
  pub tally: Item<'a, u128>;
}
```

This means we'll have to update our `Cw721Contract` struct in several places as we add the `Item` type to it.

`Item`s are created via the `new` implementation, and require a unique string to identify them. One way it is used is for storing local configuration information about a contract:
```rust
pub struct Config {
    pub owner: Addr,
}
pub const CONFIG: Item<Config> = Item::new("config");
```

We'll see more of `Item` later in the course.

# Exercise
There are two things to do, update the lifetimes and wrap the elements in the `Item` type.

1. Add a lifetime annotation labeled a `'a` to the main struct.
2. Wrap each member type of the struct in an `Item` so that it as the same lifetime.

# Starter
```rust
use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
pub struct Cw721Contract // add a lifetime label here
{
    // wrap the types (all three) in an Item object:
    pub contract_info: ContractInfoResponse,
    pub minter: Addr,
    pub token_count: u64,
}
```

# Answer
```rust
use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
pub struct Cw721Contract<'a>
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub token_count: Item<'a, u64>,
}
```