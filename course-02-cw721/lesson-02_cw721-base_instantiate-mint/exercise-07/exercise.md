---
main file: execute.rs
supporting files: contract.rs, cw721/query.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

# Approvals
This is simple exercise, getting you accustomed to the members of the `TokenInfo` field.

The field we'd like to highlight in this exercise is `approvals`. Its contents are not supplied by `MintMsg`, rather, the `Vec<Approval>` is initialized with an empty vector. An `Approval` takes a `sender` account who is allowed to transfer or send the NFT token and `expiration` field that tells the contract when the `Approval` expires. The `approvals` vector is updated via the `ExecuteMsg::Approve` based on the CW721 spec:
```rust
ExecuteMsg::Approve { spender, token_id, expires, } 
```

# Exercise

1. Create the `TokenInfo` object and assign it to a `token` variable.
2. For the `owner` member, directly validate the `&msg.owner` address.
3. For the `approvals` field, assign an empty vector with `vec![]`.
4. For `token_uri` and `extension`, assign the corresponding `msg` fields.

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

  // create the token here
  
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

  Ok(Response::new()
      .add_attribute("action", "mint")
      .add_attribute("minter", info.sender)
      .add_attribute("owner", msg.owner)
      .add_attribute("token_id", msg.token_id))
}
```