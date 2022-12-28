---
main file: contract.rs
supporting files: state.rs, cw721/query.rs
cosmwasm topics: Item, CW2,
rust topics: '?'
---

# Overview
> 

# ContractInfoResponse
Now that we've saved the contract info from the CW2 spec point of view, let's save that information directly to our contract so it can be recalled by curious external contracts.

You might recall for yourself that we already worked with the CW721 spec struct, `ContractInfoResponse`. We'll be using that here.
```rust
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}
```

As part of our recreating the `Cw721Contract` struct, we used it as the type for the `contract_info` member:
```rust
pub struct Cw721Contract<'a>
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    // ... other members
}
```

Since it is of type `Item`, we can make use of its `impl` functions to act on it. For this exercise, we're going to `save` the item. 

## Save Impl
The `impl` for `Item::save` looks like this:
```rust
pub fn save(&self, store: &mut dyn Storage, data: &T) -> StdResult<()> {}

// Used like so:
let mut deps = mock_dependencies();
let game_name: Item<String> = Item::new("game_info");
game_name.save(deps.storage, &"Race for the Galaxy".to_string())
```

# Exercise
With `InstantiateMsg`, the creator of the contract will provide us with the name and symbol.

1. Create a let variable called 'info'.
2. Assign the `ContractInfoResponse` struct to it (on two lines).
3. Populate the struct with `msg.name` and `msg.symbol` items (in between the struct braces).
4. Save the `info` struct (as a reference) to the blockchain using the the `contract_item` object.
    - You'll need to use `self` to refer to it. This is because `cw721-base` defines `instantiate` as part of an `impl` for the `Cw721Contract` struct.
    - `save` returns `StdResult<()>`, so don't forget about adding the `?` helper. 

> ### The `contract_info` key
> Last lesson, we learned that the CW2 spec reserved the string "contract_info" for its own use of storing its own contract info. To avoid an conflicts, the `cw721-base` contract defines this `contract_info` `Item` type with a different string: "nft_info". Whew! 😓

# Starter
```rust
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = "0.16.0";
pub fn instantiate(
    &self,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<C>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // add struct here (should take up four lines)
    // save the info reference

    Ok(Response::default())
}
```

# Answer
```rust
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = "0.16.0";
pub fn instantiate(
    &self,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<C>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let info = ContractInfoResponse {
      name: msg.name,
      symbol: msg.symbol,
    };
    self.contract_info.save(deps.storage, &info)?;

    Ok(Response::default())
}
```