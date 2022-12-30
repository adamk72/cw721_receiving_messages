---
main file: execute_fns.rs
supporting files: cw721-base/execute.rs, cw721/receiver.rs, contract.rs, cw721/query.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

Now that we've setup the ancillary parts of the portal contract so that we can validate a user and possibly prevent Jump Ring travel if the visa hasn't been validated yet, let's get into the heart of the lesson with the `ReceiveNft` function.

In the last lesson, we went over the implementation of `Cw721ReceiveMsg`. In the following exercises, we'll make use of that to capture the data sent to us via `WasmMsg::Execute`.

# How it Works

We already saw how the `ExecuteMsg::SendNft` from the `cw721-base` contract work, sending a `Cw721ReceiveMsg` message via its `into_cosmos_msg` function. Now we're going to write the code of how we'll receive that message.

If you'll recall as part of the `cw721` spec code, in `into_cosmos_msg`, what we did was serialize and send `ReceiverExecuteMsg::ReceiveNft()`. Through the Cosmos SDK system on the blockchain, that message (addressed to a contract) is received by the `execute` entry point of the smart contract. There, the `match` determines which arm to follow through on. In our case, we called it `receive_visa`. 

So now, let's implement `receive_visa`. We'll do this in three major parts:
- Query the sending contract so we can get some data on the visa NFT.
- Validate the results of the query to ensure that the traveler is allowed to use the Jump Ring.
- Update the `VISAS` list, making sure to set the `visa.approved` to true (assuming the query was positively validated!).

# Cw721QueryMsg
We'll start with learning about the `Cw721QueryMsg` `enum` from the `cw721` spec code. It's a list of all the query responses expected to be implemented by a CW721 compliant NFT contract such providing information on the approvals, the contract info, and the tokens and their metadata. We're interested in this part of it:
```rust
NftInfo { token_id: String },
```

# Exercise

1. With a `msg` variable, assign `Cw721QueryMsg::NftInfo`. It will take a cloned `token_id`. 

# Starter
```rust
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, 
};
pub fn receive_visa(
    sender: String,
    token_id: String,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // add msg here (three lines)

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```

# Answer
```rust
use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Response, 
};
pub fn receive_visa(
    sender: String,
    token_id: String,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let msg = Cw721QueryMsg::NftInfo {
        token_id: token_id.clone(),
    };

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```