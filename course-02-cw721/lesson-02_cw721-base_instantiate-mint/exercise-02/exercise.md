---
main file: contract.rs 
supporting files: cw2-lib.rs 
cosmwasm topics: CW2, Item
rust topics:
---

# Overview
> The revered Exodite scientist Dr. Palicki was the first sapient in the InterChains to notice that the Jump Rings weren't always programmed identically. Her standards proposal stabilized Jump Ring travel.

# CW2
Yes, *another* standard to be aware of! The CosmWasm community has standard to define contract info for the purposes of managing contract-to-contact migrations, which is discussed in detail [here](https://docs.cosmwasm.com/cw-plus/0.9.0/cw2/spec/)<ExternalLink>.

For our purposes, the CW2 specification provides a function that we need so we can ensure we're compliant across the CosmWasm contract ecosystem. The function is called `set_contract_version`;

The function `set_contract_version` takes three parameters: the `deps.storage` and a strings for the contract name and version number. 

## Storage Details
⚠️ The contract info is stored in an `Item` type under the storage key of "contract_info", so be careful not to make use of that when creating your own `Item` objects while following the CW2 specification.

The struct that stores the data looks like so:
```rust
pub struct ContractVersion {
    pub contract: String,
    pub version: String,
}
```

# Exercise
We've set up the basic boilerplate for a CosmWasm `instantiate` function. You'll need to add the `set_contract_version`.

1. Add the appropriate `use` from the `cw2` specification.
2. Add `set_contract_version`, using `deps` and the provided `const` values for the contract name and version number.

# Starter
```rust
// import with 'use' here

const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = "0.16.0";
pub fn instantiate(
    &self,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<C>> {

    // add function here

    Ok(Response::default())
}
```

# Answer
```rust
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = "0.16.0";
pub fn instantiate(
    &self,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<C>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}
```