---
main file: execute_fns.rs
supporting files: msg.rs, cw761/receiver.rs, error.rs
cosmwasm topics:
rust topics:
---

# Overview
> 

# Some Rationale
In the realm of the Exodite cluster, in order for someone to visit one of their eco-worlds, they have to fill out a visa application form. This takes the form of some administrative contract sending a message to the destination Jump Ring portal contract via the `ExecuteMsg::AssignVisa` message. This is *not* the act of sending an NFT token over, but rather a preliminary step to whitelist the applicant and their NFT for future validation.

Anyone who has not had their visa NFT submitted through this process will not be allowed to successfully call the `ExecuteMsg::JumpRingTravel` message. You'll see how all of this comes together as we bring the parts together, starting with the `assign_visa` function.

# Implementation Details
The `assign_visa` function is by itself simple; there are a just a few structs we need to assemble to make it work.

Since `ExecuteMsg::AssignVisa` is a message, we need to pass a serializable informational parameter. We're calling that `AssignVisaMsg`. Along with that, we have the `VisaAdminDetails` which holds the details we're most interested in for later validation purposes. `VisaAdminDetails` is used as a component of both `AssignVisaMsg` and the last struct, simply `Visa`. You can see the whole layout in the `msg.rs` tab, but here is the `Visa` struct in particular:
```rust
pub struct Visa {
    pub approved: bool,
    pub details: VisaAdminDetails,
}
```

When the visa is assigned, `approved` will be set to `false`. As long as this is `false`, then the visa holder cannot travel to the specified Jump Ring. How do they get approved? By sending their NFT visa token over to the destination Jump Ring portal contract via the `SendNft` function that is part of `cw721-base`. 

# Storing the Visas
Last point of note before we do a little coding. We'll be storing the incoming `Visa`s in a `cosmwasm-std` `Map` object called `VISAS`. See the `state.rs` tab for details; we won't go over that again since we've done it a couple of times already. This will act as our whitelist once the NFT transfer as succeeded and been approved.


# Exercise
We'll wrap up `assign_visa` in one exercise. You should be familiar enough with the details from previous exercises.

1. Create a variable called `visa` and assign a `Visa` struct.
    - The `approved` field will be set to `false`.
    - The `details` will be cloned from `msg.details`.
2. Save the new `&visa` reference to `VISAS`. The 'key' parameter will come from `&msg.details.holder`. 

That's it!

# Starter
```rust
pub fn assign_visa(
    msg: AssignVisaMsg,
    deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {

    // set the visa variable here (four lines)

    // save the visa to VISAS here.

    Ok(Response::new().add_attribute("action", "assign_visa"))
}

```

# Answer
```rust
pub fn assign_visa(
    msg: AssignVisaMsg,
    deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {

    let visa = Visa {
        approved: false,
        details: msg.details.clone(),
    };

    VISAS.save(deps.storage, &msg.details.holder, &visa)?;

    Ok(Response::new().add_attribute("action", "assign_visa"))
}

```