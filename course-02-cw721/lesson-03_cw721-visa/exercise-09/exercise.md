---
main file: one file name
supporting files: comma separated list
cosmwasm topics:
rust topics:
---

# Overview
> 

# Execute and Query
We're not going to do much with `execute` and `query` functions for this course. We do want to emphasize how you can call "down" to the `cw721-base` contract so you can leverage function there without having to write very much code. The base contract will handle all the main functionality that supports the CW721 spec. It also includes `mint` which you could replace with your own version.

Recall a couple of exercises ago that we called the default `instantiate` function like so?
```rust
Cw721VisaContract::default().instantiate();
```
Do the same here, but this time for `execute` and `query`. Nothing else needs to be done.

# Exercise

1. Call the corresponding default functions using `Cw721VisaContract`. The parameters match the input of their respective `entry_point` functions.  

# Starter
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
type Cw721VisaContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "cw721-visa";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // call the default execute
    Cw721VisaContract::default().execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // call the default query
    Cw721VisaContract::default().query(deps, env, msg)
}

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
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Cw721VisaContract::default().execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Cw721VisaContract::default().query(deps, env, msg)
}

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