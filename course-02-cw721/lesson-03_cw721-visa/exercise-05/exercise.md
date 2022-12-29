---
main file: contract.rs
supporting files: msg.rs, state.rs, help-1
cosmwasm topics:
rust topics:
---

# Overview
> 

Now we'll start to dive into the meat of the `instantiate` function for the `cw721-visa` contract. This function will be relatively simple. Our task for the next few exercises is to:
1. Get and save the configuration information for the newly instantiate contract.
2. Call the default `instantiate` from `cw721-base` so we can leverage that.
3. Save the contract info following the CW2 specification.

As a challenge, we're going to leave you on your own for much of this. If you need help, look at the Help section and the `msg.rs` and `state.rs` tabs. 

# Exercise

1. Create a variable, `config`, and populate it with a `Config` object using `msg` details. You'll need to clone `msg.apes` field.
2. Save `&config` using the `CONFIG` `Item`.

# Starter
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    // create the config

    // save the config

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```

# Answer
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        jump_ring: msg.jump_ring,
        apes: msg.apes.clone(),
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```