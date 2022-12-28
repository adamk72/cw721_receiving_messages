---
main file: exercise-04
supporting files: cw721/query.rs
cosmwasm topics:
rust topics:
---

# Overview
>  For an Exodite Administrator of Planetary Experience (an "Ape"), contracts are sacrosanct.

# Contract Info
One of the queries required in the CW721 spec is `ContractInfo`. We'll need to add a response member to our `Cw721Contract` struct so that anyone inquiring about details can get them.

A query from another contract to our contract's `ContractInfo` will return the following details (which you can find in the `cw721` spec `query.rs` file):

```rust
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}
```

# Exercise

1. Add the appropriate `use` to reference `ContractInfoResponse` from the `cw721` package.
2. Above `minter`, add a new member called `contract_info` of type `ContractInfoResponse` 

# Starter
```rust
use cosmwasm_std::Addr;
// Add a use statement for 
pub struct Cw721Contract
{
    pub // add the response member here
    pub minter: Addr,
    pub token_count: u64,
}
```

# Answer
```rust
use cosmwasm_std::Addr;
use cw721::ContractInfoResponse;
pub struct Cw721Contract
{
    pub contract_info: ContractInfoResponse,
    pub minter: Addr,
    pub token_count: u64,
}
```