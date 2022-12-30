---
main file: one file name
supporting files: comma separated list
cosmwasm topics:
rust topics:
---

# Overview
> 

# Cw721ReceiveMsg
Before we finish `send_nft`, we're going to re-create `Cw721ReceiveMsg`. In an earlier lesson, we provided you details on the struct itself:
```rust
pub struct Cw721ReceiveMsg {
    pub sender: String,   // the current owner of the NFT token
    pub token_id: String, // the id of the token
    pub msg: Binary,      // additional info; we won't be using this.
}
```

But there's more to `Cw721ReceiveMsg` than just this; it also implements a couple of utility functions to make sending CosmWasm messages of itself easy. This pattern really makes `send_nft` (and any other functions that need to use `Cw721ReceiveMsg`) a lot easier to write and read.

## ReceiverExecuteMsg
The first item of note in this exercise is that the bottom of the boilerplate:
```rust
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}
```
This `enum` is an internal helper for proper serialization of our first `impl` function, `into_binary`. In the next exercise, we'll write up `into_cosmos_msg` which is main function we'll using in other code.


# Exercise
We'll write `into_binary` first. This function serialized the message to be passed on.

1. Write the function signature out:
    - It's a public function called `into_binary`.
    - It takes `self` for its parameter (since it's an `impl` function).
    - It returns `StdResult<Binary>`.
2. In the body, create a variable called 'msg' and assign it `ReceiverExecuteMsg::ReceiveNft(self)`. 
3. Lastly use `to_binary` on `&msg` and close the body bracket.


# Starter
```rust
use schemars::JsonSchema;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, StdResult, WasmMsg};

impl Cw721ReceiveMsg {
    // add into_binary here (four lines)
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
    
}

#[cw_serde]
enum ReceiverExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
}
```