---
main file: one file name
supporting files: (see previous), Species
cosmwasm topics:
rust topics:
---

# Overview
> 

Recall that we wrote up a function, `assign_visa`, which added `Visa` object with an `approved` field set to false. Now that we've gone through the effort to validate that the NFT sender is of the correct `SapienceLevel` to visit the world through this Jump Ring portal, let's add them to the `VISAS` list.

We went over `Map::update` a couple of lessons ago, but it's just complicated enough that we think it's worth a review. `update` has the following signature:

```rust
pub fn update<A, E>(&self, store: &mut dyn Storage, key: K, action: A) -> Result<T, E> {};
```

The interesting parameter is the `action` one. It takes in a function (a [closure](https://doc.rust-lang.org/rust-by-example/fn/closures.html)<ExtneralLink> works) and acts on it to return an `Option` which we can match and respond to. 

In the case of our `VISAS` list, if we get a `None`, we'd like to reject the request because the visa application wasn't found, otherwise, we'll update the visa that is returned to us (in a `Some`) so that it is approved, i.e., `visa.approved = true`. 

# Exercise
We've set up the basic structure of the function; you just need to fill in the responses to the `match` expression. 

1. `None` is simple; return an `Err` using the `NotOnList` error variant.
2. `Some` will take a `mut visa` variable. On new lines:
    - Set the visa to be approved.
    - Finish with `Ok(visa)`.

That finished the `receive_visa` function; now anyone passing these checks can call `ExecuteMsg::JumpRingTravel` on this contract successfully.

This also finishes this course!

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
    if incoming_sapience_level.as_num() < contract_min_sapience.level.as_num() {
        return Err(ContractError::NotSmartEnough {});
    }

    VISAS.update(deps.storage, &Addr::unchecked(sender), |old| match old {
        // add the None response here
        // add the Some response here (four lines)
    })?;

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

    VISAS.update(deps.storage, &Addr::unchecked(sender), |old| match old {
        None => Err(ContractError::NotOnList {}),
        Some(mut visa) => {
            visa.approved = true;
            Ok(visa)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}
```
```