---
main file: receiver.rs
supporting files: cw-std/cosmos_msg.rs 
cosmwasm topics:
rust topics: Into trait
---

# Overview
> 

# IntoCosmosMsg
Now we'll create the main function, `into_cosmos_msg`. We're adding a boilerplate for the function signature in the exercise, so let's go over it here:
```rust
pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>> {}
```
As an `impl`, it takes `self` of course, and then `contact_addr` of type `Into<String>`. `Into` is a Rust trait that in this case, expects whatever is passed to it be able to convert to a `String` type.

It returns `CosmosMsg` which we've discussed before in the *Intro to CosmWasm* course, but we'll highlight it again here. `CosmosMsg` is an enum that accepts a variety of messages.
```rust
pub enum CosmosMsg<T = Empty> {
    Bank(BankMsg), // For account balances and fund transfers
    /// a few others
    Wasm(WasmMsg), // For wasm contracts
    Gov(GovMsg),   // For governance features like voting and proposals
}
```

The message we're interested in is the `Wasm` variant and in particular, `WasmMsg::Execute`. See the `cosmos_msg.rs` tab for more a more complete look of `WasmMsg` in general.

# WasmMsg::Execute
`WasmMsg::Execute` takes a contract address string, a binary message, and a funds ('Coin') vector. It dispatches a call to another contract at the provided address. 
```rust
Execute {
    contract_addr: String,
    msg: Binary,
    funds: Vec<Coin>,
},
```

# Exercise

1. Convert the `Cw721ReceiveMsg` itself into binary (reminder: call `self.into_binary()?`).
2. Create an `execute` variable and assign it the `WasmMsg::Execute` variant.
    - Use `.into` on the `contract_addr`. 
    - `msg` can be left as is.
    - `funds` is an empty vector (`vec![]`).

# Starter
```rust
use schemars::JsonSchema;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult, WasmMsg};

impl Cw721ReceiveMsg {
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveNft(self);
        to_binary(&msg)
    }

    pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        // add msg here
        // add execute here

        Ok(execute.into())
    }
}

#[cw_serde]
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}
```

# Answer
```rust
use schemars::JsonSchema;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult, WasmMsg};

impl Cw721ReceiveMsg {
    pub fn into_binary(self) -> StdResult<Binary> {
        let msg = ReceiverExecuteMsg::ReceiveNft(self);
        to_binary(&msg)
    }

    pub fn into_cosmos_msg<T: Into<String>, C>(self, contract_addr: T) -> StdResult<CosmosMsg<C>>
    where
        C: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        let msg = self.into_binary()?;
        let execute = WasmMsg::Execute {
            contract_addr: contract_addr.into(),
            msg,
            funds: vec![],
        };
        Ok(execute.into())
    }
}

#[cw_serde]
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}
```