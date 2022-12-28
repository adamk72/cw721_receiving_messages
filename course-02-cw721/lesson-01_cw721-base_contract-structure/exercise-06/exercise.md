---
main file: state.rs
supporting files: 
cosmwasm topics:
rust topics:
---

# Overview
> Not just anyone can enter the Exodite cluster. You have to apply for a visa to be considered to bless your eyes and locomotive appendages with their illustrious worlds.

# Operators
In the `cw721-base` contract, they define the concept of `operators` who are entities that are specially privileged to make changes to the contract. Since there can be more than one operator, they are stored in a list-like object called a `Map`. 
        
# The `Map` Struct
Whereas `Item` stores a single datum, `cw-storage-plus` provides the `Map` object for storing multiple related data. It looks like this:
```rust
pub struct Map<'a, K, T> {
    namespace: &'a [u8],
    key_type: PhantomData<K>,
    data_type: PhantomData<T>,
}
```
The `namespace` member allows for maps to be distinguished between one another on the blockchain. Adding onto our example from the last exercise, you might implement it like this:
```rust
use cw_storage_plus::{Item, Map};
pub struct Rider { pub name: String };

struct RacesContract<'a> {
  pub riders: Map<'a, &'a u64, &'a Rider>;
  pub tally: Item<'a, u128>;
}
// ...
pub const RACE_LIST: Map<u64, Rider> = Map::new("race_list");
// ...
RACE_LIST.save(deps.storage, rider.id, &rider)?;
```

# `Operators` and `Map`
In our example, our first type, the `key_type` is of type `u64`. However, it can be more complex, in the case of the `Operators` map, the key is a tuple, consisting of two `Addr` structs, like so:
```rust
(&'a Addr, &'a Addr)
```
The first address is that of the granter; the second is that of an operator who gains full control of the granter's account. This allows the operator to act on behalf of the granter.

# Exercise
This exercise is best approached in stages.

1. Add an `operators` member below the `token_count` member. 
2. Add a type of `Map<'a, ,>` to start, where the first parameter is the lifetime, `'a`.
3. For the second parameter, add the dual `Addr` tuple mentioned above.
4. For the third parameter, add the type `Expiration`. 

`Expiration` is defined as an enum by the `cw-utils` package. See the `expiration.rs` tab for what it looks like. We don't make use of it in this course, but it is worth being aware of.

# Starter
```rust
use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
pub struct Cw721Contract<'a>
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub token_count: Item<'a, u64>,
    // add the operators map here.
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
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
}
```