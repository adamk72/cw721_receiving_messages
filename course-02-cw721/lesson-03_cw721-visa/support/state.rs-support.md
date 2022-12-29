---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
Write summary here

In general, we learned about:

From the `cosmwasm` libraries, we touched on:

## Next Up

<!-- This should be the contract file -->
# Answer
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub apes: Vec<Addr>,
    pub jump_ring: Addr,
    // pub extension: Extension,
}

pub const CONFIG: Item<Config> = Item::new("config");
```