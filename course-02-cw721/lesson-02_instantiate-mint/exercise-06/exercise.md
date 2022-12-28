---
main file: execute.rs 
supporting files: ./contract.rs, cw721-base/error.rs
cosmwasm topics: @todo Rejection (see below)
rust topics:
---

# Overview
> The Exodites are a very hierarchical society. That's not to say the individuals can't move between cultural strata, but it can be challenging for all but the brightest.

# Minter, again
In our last exercise, we got the minter's address, validated it via `deps.api` and then saved it to our `self.minter Item`:

```rust
self.minter.save(deps.storage, &minter)?;
```

Loading a single `Item` is trivial. Simply call the `load` function with only the storage object. Here's its function signature:
```rust
pub fn load(&self, store: &dyn Storage) -> StdResult<T> {}
```

In this exercise, we'll load the `minter` address so we can verify the the sender is who they say they are. We'll need to make use of `ContractError` in order to reject any unauthorized attempts to mint. See the `error.rs` tab for a refresher of what contract errors look like; these are for the `cw721-base` contract in particular.

@todo: Elaborate on how the rejection looks to the caller (?)

# Exercise

1. Load the minter into a variable called 'minter' using the `Item::load` function.
2. Compare the `info.sender` against the loaded `minter` result (this is a `!=` comparison).
3. If it fails, reject by returning an `Err` with the `Unauthorized` variant. 

# Starter
```rust
pub fn mint(
  &self,
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: MintMsg<T>,
) -> Result<Response<C>, ContractError> {

  // get the minter
  // check the sender is the same a the minter (three lines total)

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
  
  Ok(Response::new()
      .add_attribute("action", "mint")
      .add_attribute("minter", info.sender)
      .add_attribute("owner", msg.owner)
      .add_attribute("token_id", msg.token_id))
}
```