---
main file: execute_fns.rs
supporting files: (see previous), cw-std/wasm.rs, cw-std/mod.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

In this exercise, we'll revisit `WasmQuery` from the `cosmwasm_std` package. 

## WasmQuery::Smart
The <ExternalLink href="https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.WasmQuery.html">WasmQuery</ExternalLink> enum has two arms, `Smart` and `Raw` query. As we did in the *Intro to CosmWasm* course, we're going to use `Smart` query, which contains two properties:
```rust
Smart {
    contract_addr: String,
    msg: Binary, /// msg is the json-encoded QueryMsg struct
},
```
`contract_addr` will take a string address (or an `Addr` type converted using `to_string()`) and `msg` will take the actual query message itself, which is what we'll build in this exercise.

Once we've created the query, we'll apply it using the `deps.querier.query` function.

# Exercise

1. Add a `query` variable. Assign `WasmQuery::Smart` to it.
  - `contract_addr` is of type `String`; use the `sender` from the `info` parameter.
  - `msg` require a binary type; apply `to_binary` to the *reference* of the `msg` variable storing the `NftInfo` message. 

# Starter
```rust
use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, Response, 
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

    // add query here (four lines)

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```

# Answer
```rust
use cosmwasm_std::{
    to_binary, DepsMut, Env, MessageInfo, Response, 
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

    let query = WasmQuery::Smart {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&msg)?,
    };

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```