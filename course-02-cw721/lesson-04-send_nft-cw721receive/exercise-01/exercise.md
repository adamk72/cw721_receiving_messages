---
main file: execute_fns 
supporting files: ./help-1.rs 
cosmwasm topics: Response
rust topics:
---

# Overview
> 

# SendNft
For this exercise, we're going to have you write a portion of the `ExecuteMsg::SendNft` variant function, `send_nft` that is part of the `cw721-base` contract. This is so you can get hands on feel for what is happening with the `Cw721ReceiveMsg` structure which we'll build in full in the later exercises.

`send_nft` has three simple steps:
1. Transfer the NFT to the new contract &mdash; this simply means updating the `tokens` list with the new associated contract address.
2. Prepare the `Cw721ReceiveMsg` object.
3. Send the message as part of  `Ok(Response)`. 

## Transfer NFT
You'll see in the exercise boilerplate what there's an initial step, `_transfer_nft`. This is a helper function called by a couple of `cw721-base` functions including the main one called from the `ExecuteMsg::TransferNft` variant arm). It has a lot of steps to it, so we'll forego the details for this lesson, but what it does is validates that the sender is either the owner or on the approval list and saves the `token_id` to the `tokens` list (see Help for more).

# Response
We've seen `Response` plenty of times before, here and in the *Intro to CosmWasm* course. It's been awhile since we've discussed it though. As reminder, `Response`, is part of the `cosmwasm_std` library and is a wrapper around CosmWasm's [Event](https://docs.cosmwasm.com/docs/1.0/smart-contracts/events/) system that handles the messaging. This is how the contracts communicate with one another. 

Most commonly seen is the `.add_attribute()` implementation, which sends basic information to the blockchain for later reference/data collection. For our case here, we'll be interested in `.add_message()` for communication across contracts. See the Help for a reminder about the Actor model design pattern used by CosmWasm.

# Exercise
This is a preparatory exercise, setting up for future work.

1. Add the use import for `Cw721ReceiveMsg` from the `cw721` spec package. We'll use it in the next exercise.
2. Add `.add_message()` to the `Response` object. For now, leave the parameter list blank. 

# Starter
```rust
// Add use here
fn send_nft(
    &self,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    token_id: String,
    msg: Binary,
) -> Result<Response<C>, ContractError> {
    self._transfer_nft(deps, &env, &info, &contract, &token_id)?;

    Ok(Response::new()
        // add_message here
        .add_attribute("action", "send_nft")
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", contract)
        .add_attribute("token_id", token_id))
}
```

# Answer
```rust
use cw721::Cw721ReceiveMsg;
fn send_nft(
    &self,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    token_id: String,
    msg: Binary,
) -> Result<Response<C>, ContractError> {
    self._transfer_nft(deps, &env, &info, &contract, &token_id)?;
    
    Ok(Response::new()
        .add_message(send)
        .add_attribute("action", "send_nft")
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", contract)
        .add_attribute("token_id", token_id))
}
```