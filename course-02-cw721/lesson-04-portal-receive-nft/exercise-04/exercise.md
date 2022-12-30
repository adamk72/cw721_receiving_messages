---
main file: execute_fns.rs
supporting files: msg.rs, state.rs, cw-storage/map.rs, error.rs
cosmwasm topics: Map::load
rust topics: match
---

# Overview
> 

# Fixing up JumpRingTravel
The original `ExecuteMsg::JumpRingTravel` function, `initiate_jump_ring_travel`, didn't do much. We'll change that up here. 

Anyone calling `JumpRingTravel` will not be allowed through if they have not yet been approved. This means we'll have to pull up the specific traveler from the `VISAS` list and check to see if their `Visa` has been approved.

## Map Load
From the `cw_storage_plus` package, we've both saved and loaded information from `Item` types, but have only saved to `Map` types. In this exercise, we'll call the `load()` function for `Map` from the `VISAS` list.

Normally, this is pretty straightforward. See loading the tokens from `cw721-base`: 
```rust
let info = self.tokens.load(deps.storage, &token_id)?;
```

The `?` operator returns `Visa` or a `StdError` if it can't find anything. We'd like to make that error message a little more helpful, so we're going to wrap it in a `match` and return a specific error message.

### match
Recall that `match` takes a form like this:
```rust
match thing {
  Ok(v) => v,
  Err(_) => return Err(),
}
```
The match arms aren't important for this example (though take note, as you'll use them in the exercise). It's the `thing` that we're interested in.

The `thing` variable can be anything that outputs something to be matched... for instance the results of a call to `Map::load()`. This means, replacing `thing` with our tokens call, we can do this:
```rust
match self.tokens.load(deps.storage, &token_id) {
  Ok(v) => v,
  Err(_) => return Err(),
}
``` 

# Exercise
Like with `assign_visa`, we can get this tone in one exercise:

1. Create a variable called `visa` and assign it the results of a `match` on `VISAS.load`.
    - The 'key' parameter to load on will be `&info.sender`.
    - Use a simple `v` variable and to capture and exit the `Ok` arm.
    - For the `Err(_)` arm, return from the function with another `Err` sending `ContractError::NotOnList {}`. 
2. If the `visa` is not approved, return an `Err` with `ContractError::Unapproved {}`.  

# Starter
```rust
pub fn initiate_jump_ring_travel(
    _to: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    // find the visa in question here (four lines)

    // check to see if the visa is approved (three lines)

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", &info.sender))
}
```

# Answer
```rust
pub fn initiate_jump_ring_travel(
    _to: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let visa = match VISAS.load(deps.storage, &info.sender) {
        Ok(v) => v,
        Err(_) => return Err(ContractError::NotOnList {}),
    };

    if !visa.approved {
        return Err(ContractError::Unapproved {});
    }

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", &info.sender))
}
```