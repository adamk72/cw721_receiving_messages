---
main file: execute_fns
supporting files: (see previous), query.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

In order to make our query happen, we need to pass it on to blockchain using the `deps.querier.query` method.
As the name implies, this queries the contract we're interested in getting information from (which is known via the `contract_addr` field in `WasmQuery::Smart`).

# QueryRequest
The `deps.querier.query` takes a reference to `QueryRequest`. For our purposes, we're interested in passing our `Smart` query over, which `QueryRequest` conveniently supports (see `mod.rs` for details). An example of using this looks like this:
```rust
let query = WasmQuery::Smart {
    contract_addr: portal.to_string(),
    msg: to_binary(&QueryMsg::MinimumSapience {})?,
};

let res: SapienceResponse = deps.querier.query(&QueryRequest::Wasm(query))?;
```

# NftInfoResponse
The response from a query is known by the developer, but not explicitly by the code. It returns a `StdResult` which we can unwrap (via `?`) into a known type. The response back for a query to `NftInfo` is unsurprisingly, `NftInfoResponse`:
```rust
pub struct NftInfoResponse<T> {
    pub token_uri: Option<String>,
    pub extension: T,
}
```
Since we're using the `Extension` feature of `cw721-base`, our `T` generic will be `VisaMetadata`. 

# Exercise

1. Add a `res` variable of type `NftInfoResponse<VisaMetaData>`. Pass the `query` to a reference to `QueryRequest` all via the `query` method of `deps.querier`.

This will fit on one line. 

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

    let query = WasmQuery::Smart {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&msg)?,
    };

    // add res here

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

    let res: NftInfoResponse<VisaMetadata> = deps.querier.query(&QueryRequest::Wasm(query))?;

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```