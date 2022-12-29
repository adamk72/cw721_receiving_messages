---
main file: one file name
supporting files: comma separated list
cosmwasm topics:
rust topics:
---

# Overview
> 

# Return to CW2
Almost done with the `instantiate` function. The last thing we need to do is set the contract version in order to keep with the CW2 specification. As a reminder, the reason for this is is to ensure that events like contract migrations happen successfully.

This has to happen *after* the call to the default `instantiate` since it is also called there. This allows our contract to override the default information from the `cw721-base` contract.

# Exercise

1. Add the import for the `cw2` spec so we can use `set_contract_version`.
2. Set the contract name and version, using the provided `const`s. 

# Starter
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
type Cw721VisaContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
// add use for cw2

const CONTRACT_NAME: &str = "cw721-visa";
const CONTRACT_VERSION: &str = "0.1.0";

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

    let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: msg.apes[0].to_string(), 
    };

    Cw721VisaContract::default().instantiate(
        deps.branch(),
        env,
        info,
        cw721_base_instantiate_msg,
    )?;
    
    // set the contract version here.

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```

# Answer
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
type Cw721VisaContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "cw721-visa";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

    let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: msg.apes[0].to_string(), 
    };

    Cw721VisaContract::default().instantiate(
        deps.branch(),
        env,
        info,
        cw721_base_instantiate_msg,
    )?;
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```