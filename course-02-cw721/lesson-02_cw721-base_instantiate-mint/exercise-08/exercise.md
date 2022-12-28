---
main file: execute.rs
supporting files: cw721/query.rs, cw721-base/state.rs
cosmwasm topics: IndexedMap
rust topics:
---

# Overview
> 

# Updating the Tokens List
Now that we have described the token, filling in the `TokenInfo` with details from the `MintMsg` that was passed to the `mint` function, let's actually save the new token to the blockchain.

This is done via our `tokens` list object from `Cw721Contract`:
```rust
pub struct Cw721Contract<'a, T, C, E, Q>
{
    pub token_count: Item<'a, u64>,
    pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a, T>>,
}
```
Each time we add a token, we update the `tokens` list via `IndexedMap::update()` and increment the `token_count` variable. 

The simplified signature for `IndexedMap::update()` is the following:

```rust
pub fn update<A, E>(&self, store: &mut dyn Storage, key: K, action: A) -> Result<T, E> {};
```
And works like so:

```rust
let horse = Horse { name: "Seabiscuit", rider: "George Woolf" }
self.horses
    .update(deps.storage, &horse.name, |old| match old {
        Some(_) => Err(ContractError::AlreadyAdded {}),
        None => Ok(horse),
    })?;
```

`IndexedMap` store each item as a `Some`. The `update` function takes an `action` function parameter that lets the caller respond to the `Some` or `None` response as a result of checking to see if the item is already listed. In our case, a `Some` response is rejected by an error message. 


# Exercise
We added `increment_tokens` already, so you won't need to implement that but can see it in the `state.rs` tab.

1. Following the example above, have `self.tokens` update itself with the new `token` that was created. Just like the example, it should take up five lines.
2. Use the `msg.token_id` for the `key` parameter and the full `token` variable for the `Ok()` response.


# Starter
```rust
pub fn mint(
  &self,
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: MintMsg<T>,
) -> Result<Response<C>, ContractError> {
  let minter = self.minter.load(deps.storage)?;

  if info.sender != minter {
      return Err(ContractError::Unauthorized {});
  }

  let token = TokenInfo {
      owner: deps.api.addr_validate(&msg.owner)?,
      approvals: vec![],
      token_uri: msg.token_uri,
      extension: msg.extension,
  };

  // update the token list (five lines total)

  self.increment_tokens(deps.storage)?;

  Ok(Response::new()
      .add_attribute("action", "mint")
      .add_attribute("minter", info.sender)
      .add_attribute("owner", msg.owner)
      .add_attribute("token_id", msg.token_id))
}
```

# Answer
```rust
pub fn mint(
  &self,
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: MintMsg<T>,
) -> Result<Response<C>, ContractError> {
  let minter = self.minter.load(deps.storage)?;

  if info.sender != minter {
      return Err(ContractError::Unauthorized {});
  }

  let token = TokenInfo {
      owner: deps.api.addr_validate(&msg.owner)?,
      approvals: vec![],
      token_uri: msg.token_uri,
      extension: msg.extension,
  };

  self.tokens
    .update(deps.storage, &msg.token_id, |old| match old {
        Some(_) => Err(ContractError::Claimed {}),
        None => Ok(token),
    })?;

  self.increment_tokens(deps.storage)?;

  Ok(Response::new()
      .add_attribute("action", "mint")
      .add_attribute("minter", info.sender)
      .add_attribute("owner", msg.owner)
      .add_attribute("token_id", msg.token_id))
}
```