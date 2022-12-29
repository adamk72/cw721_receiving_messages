---
main file: contract.rs
supporting files: msg.rs, state.rs, cw721-base/state.rs 
cosmwasm topics: deps.branch from cw-std
rust topics:
---

# Overview
> 

# Aliasing `Cw721VisaContract`
We've add this type alias to the contract file:
```rust
type Cw721VisaContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>
```
So far, we've avoided talking a lot about the inner workings of the `Cw721Contract` structure from the `cw721-base` contract, though we've written some of the start code for it (like `InstantiateMsg` and the `mint` function). Let's deepen our dive just a little. 

As you can see in the `cw721-base/state.rs` tab, there's a lot going in just in the main struct definition, including references to `PhantomData`. Here's the beginning:
```rust
pub struct Cw721Contract<'a, T, C, E, Q> {}
```
What the generic types refer to are the following:
- `T`: The type for the metadata extension. In our case, that aliases to `VisaMetadata`.
- `C`, `E`, and `Q`: The types for the `PhantomData`. These are for later behind the scenes work; you can ignore them for the sake of this course.

For our purposes, we'll use the new type alias of `Cw721VisaContract` to make the necessary calls into the `cw721-base` contract (specifically calling `Cw721VisaContract::default().instantiate()` for this exercise). For more advanced contracts that you might write, this is a convenient shorthand to use throughout your contract.

# Deps Branching
The `cosmwasm-std` library provides a helper function to `deps` called `branch()`. This generates a new copy of the `deps` structure. We'll be using it to handle what would otherwise be a memory borrow when we call the default `cw721-base` instantiate function. Think of it as use-case specific version of `clone()`.

# Exercise

1. Call the default `instantiate()` function, using the provided `Cw721VisaContract` alias.
2. It takes the following parameters: `deps.branch()`, `env`, `info`, and `cw721_base_instantiate_msg`, 

# Starter
```rust
use cosmwasm_std::entry_point;
use crate::{ msg::InstantiateMsg, state::{Config, CONFIG}, };
use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
pub type Cw721VisaContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

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

    // call the default instantiate here (five lines)
    Cw721VisaContract::default().instantiate(
        deps.branch(),
        env,
        info,
        cw721_base_instantiate_msg,
    )?;

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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}
```