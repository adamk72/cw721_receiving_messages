---
main file: state.rs
supporting files: 
cosmwasm topics:
rust topics:
---

# Overview
>  Every year, thousands upon thousands, perhaps even even millions of visas are requested to explore planets of the Exodite cluster. Only a small percentage of applicants gain the privilege to do so.

# The `IndexedMap`
Now that we have `TokenInfo` setup, we're going to touch on two topics so we can complete our `Cw721Contract` struct with the `tokens` field.

As you might recall from when we added the `operators` field, we made use of the `Map` object from `cw-storage-plus`. `IndexedMap` is a more advanced variation of `Map` (it uses `Map` under the hood) that we need in order to properly keep track of our contract's tokens by use of a secondary index.

Like `Map`, `IndexedMap` takes a namespace and a key, along with an sub-list of indexed types:
```rust
pub struct IndexedMap<'a, K, T, I>
{
    pk_namespace: &'a [u8],
    primary: Map<'a, K, T>,
    pub idx: I,
}
```
`IndexedMap` allows us to store multiple tokens per owner. You find more specific at the CosmWasm documentation site on the [topic](https://docs.cosmwasm.com/docs/1.0/smart-contracts/state/cw-plus/#indexedmap)<ExternalLink>.

# `TokenIndexes`
So we can track tokens by owner, our `tokens` field of type `IndexedMap` require an extra index; this takes the form of `TokenIndexes`. We're just providing the high-level for you view here.
```rust
pub struct TokenIndexes<'a, T>
{
    pub owner: MultiIndex<'a, Addr, TokenInfo<T>, String>,
}
```

# Exercise
Like with `Map`, it's best to approach `IndexedMap` in stages.

1. Add a `tokens` member below the `operators` member.
2. `tokens` will be of type `IndexedMap<'a, , ,>` where the first parameter is the lifetime.
3. The second parameter will be of type `&'a str`.
4. The third parameter will be of type `TokenInfo<T>`. 
5. The fourth parameter will of type `TokenIndexes<'a, T>`.

# Starter
```rust
use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
pub struct Cw721Contract<'a>
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub token_count: Item<'a, u64>,
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub // add tokens here.
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
    pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a, T>>,
}
```