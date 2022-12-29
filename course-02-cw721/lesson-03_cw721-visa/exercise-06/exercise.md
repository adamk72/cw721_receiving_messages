---
main file: contract.rs
supporting files: msg.rs, state.rs, help-1, cw721-base/msg.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

# Using the `cw721-base` Contract
When looking at the `cw721-base` contract (and related examples from [here](https://github.com/CosmWasm/cw-nfts/tree/main/contracts)<ExternalLink>), you'll see that on import, they rebind various `use` message names to prevent any naming conflicts (and make the code easier to read). You can accomplish this in different ways, but for these lessons, we're following the idioms laid out by the CosmWasm team. So, you'll see things like this peppered through the codebase:
```rust
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
``` 

Again, use the help and the tabs to help out here. We've added the `cw721-base/msg.rs` tab to remind you of how its `InstantiateMsg` looks.

# Exercise

1. Add the `use` for `cw721-base` instantiate message structure and alias it as `Cw721BaseInstantiateMsg`.
2. Create a new variable called `cw721_base_instantiate_msg` and assign it the `Cw721BaseInstantiateMsg` object. The fields will be in this order:
    - `name` and `symbol` will be straight from the `msg` parameter.
    - `minter` will be the first ape from the incoming `msg.ape` array' use `to_string()` to match the type.

# Starter
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
// add the `use ... as` here

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

    // create the base init msg here

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```

# Answer
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;

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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```