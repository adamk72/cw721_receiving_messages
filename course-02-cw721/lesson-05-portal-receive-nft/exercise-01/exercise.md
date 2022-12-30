---
main file: contract.rs
supporting files: msg.rs, cw721/receiver.rs
cosmwasm topics:
rust topics:
---

# Overview
> Each eco-world of the Exodite cluster is a crown jewel, fit for the crown of any royalty. Yet they are open to almost any sapient, sometimes even Terrans.

# Sending NFTs Between Contracts
The CW721 spec suggests two functions to support the sending of an NFT from one contract (such as the one doing the minting) with `SendNft()` and another contact which supports `ReceiveNft()`. The destination contract doesn't have to have any of the other features of a standard NFT contract, such as approving or revoking spend permissions. It only has to be able to store limited NFT data.

You'll notice that there aren't a lot of details here. Where's the metadata, for example? The information from here will need to be stored by the contract for later reference by calling back to the originating contract, which can act a the source of truth.

# The Plan
For this lesson, we're going to go over the basics of the portal contract again, since it may have been a while for you, and then create two functions that support what we'd like to share in this lesson: `ExecuteMsg::ReceiveNft` which is mandatory for us to handle incoming NFTs properly and `ExecuteMsg::AssignVisa` a helper function we'll use so we can demonstrate more CosmWasm messaging.

First things first, let's remember what the `ExecuteMsg` enum already has on it and then we'll add those two variants over the next two exercises.

# Exercise
We've added the new variants to the `enum` in the `msg.rs` tab. You'll add them to the match expression in `contract.rs`.

1. The `AssignVisa` variant takes in a message like the other extant variants. Instead of `to`, it passes a `msg` parameter.

# Starter

```rust
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, info),
        // Add the assign_visa msg here
    }
}
```

# Answer
```rust
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, info),
        ExecuteMsg::AssignVisa { msg } => assign_visa(msg, deps, info),
    }
}
```