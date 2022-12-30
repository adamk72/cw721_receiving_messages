---
main file: one file name
supporting files: (see previous), Species, Error
cosmwasm topics:
rust topics:
---

# Overview
> 

Recall that we wrote up a function, `assign_visa`, which added `Visa` object with the `approved` field set to false. At this point, we have the info on the NFT that we'd like to host on the Jump Ring portal contract, but the visa has not been approved. We'd like some assurances our visitors won't leave behind lots of litter, so we're judging candidates based an a particular criteria.

Only Sapients of certain `SapienceLevel` will be allowed to be given visas. This was recorded in the `VisaMetadata` object when the NFT was first created in the `cw721-visa` contract. `Species` is required member of the metadata and contains the owner's `SapienceLevel` (none, low, medium, high). 

For convenience's sake, `SapienceLevel` has `as_num()` implemented on it so you can compare the enum values (which are ordinal) directly. 


# Exercise
We filled in the destructuring of the `WasmQuery` response and also inquired the contract for its minimum sapience level for you. You just need to make the comparison happen.

1. Compare `incoming_sapience_level` to `contract_min_sapience.level` using their `to_num()` function.
2. If the former is less than the latter, return an `Err` with `ContractError::NotSmartEnough {}`.   

Next up, we'll update the `VISAS` list to approve or reject the request for adding the NFT.


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

    let res: NftInfoResponse<VisaMetadata> = deps.querier.query(&QueryRequest::Wasm(query))?;

    let incoming_sapience_level = res.clone().extension.species.sapience_level;
    let contract_min_sapience: SapienceResponse =
        from_binary(&minimum_sapience(deps.as_ref()).unwrap()).unwrap();

    // compare the sapience levels here (three lines)

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

    let incoming_sapience_level = res.clone().extension.species.sapience_level;
    let contract_min_sapience: SapienceResponse =
        from_binary(&minimum_sapience(deps.as_ref()).unwrap()).unwrap();
    if incoming_sapience_level.as_num() < contract_min_sapience.level.as_num() {
        return Err(ContractError::NotSmartEnough {});
    }

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```