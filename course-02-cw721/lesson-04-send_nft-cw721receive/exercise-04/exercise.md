---
main file: execute_fns.rs
supporting files: comma separated list
cosmwasm topics: Cosmos SDK, CosmWasm
rust topics:
---

# Overview
> 

Now we can return to the `send_nft` function an finish it off. With our better understanding of how `Cw721ReceiveMsg` works, we should be able to easily see how this simple function dispatches messages to the Cosmos network... it doesn't; actually, `Cw721ReceiveMsg::into_cosmos_msg` does all of the heavy lifting and even that was relatively lightweight. The CosmWasm system of sharing messages really lies with the Cosmos SDK. 

# Cosmos SDK and CosmWasm
The [Cosmos SDK](https://docs.cosmos.network/main/intro/overview)<ExternalLink> is a blockchain framework is that is used by all of the blockchains on the Cosmos network. It helps developers create application-specific blockchains without having to delve into the low-level details of implementation. 

CosmWasm is a module that is used with the Cosmos SDK. Transactions on a blockchain are routed through the application layer and passed to the appropriate module &mdash; in our case, the CosmWasm module which handles the messaging, passing on messages to the correct contract address. See more at the CosmWasm docs website, [here](https://docs.cosmwasm.com/dev-academy/basics/cosmos-sdk-cw/)<ExternalLink>.


# Exercise

1. Create a variable called 'send' and assign it the `Cw721ReceiveMsg` struct.
    - `sender` field will be from `info.sender`. Check the type!
    - `token_id` needs to be cloned.
    - `msg` can be left as is.
2. Pass a clone of `contract` to `send`s `into_cosmos_msg`. Don't forget it returns a `StdResult` type so needs a `?`. 


# Starter
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

    // create send here 
    
    Ok(Response::new()
        .add_message(/* add send msg here*/)
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
    
    let send = Cw721ReceiveMsg {
        sender: info.sender.to_string(),
        token_id: token_id.clone(),
        msg,
    }; 
    
    Ok(Response::new()
        .add_message(send.into_cosmos_msg(contract.clone())?)
        .add_attribute("action", "send_nft")
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", contract)
        .add_attribute("token_id", token_id))
}
```