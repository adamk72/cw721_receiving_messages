---
main file: contract.rs 
supporting files: cw721/receiver.rs, msg.rs
cosmwasm topics:
rust topics: Enums
---

# Overview
> 

# Types of Enum Variants
If you're not familiar with how flexible Rust enums are, the change in the pattern for `ReceiveNft` might surprise you. Rather than the typical key-value struct of the other variants in the portal contract, `ReceiveNft` will be tuple style variant; see the `msg.rs` tab and notice it uses parentheses rather than curly braces.

To highlight this distinction, let's pull an [example](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)<ExternalLink> from the Rust doc:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

`Message::Quit` has no data associated with it, while `Message::Move` has named fields in a structure. This is what you've seen with the portal contract so far. `Message::Write` and `Message::ChangeColor` are of a tuple style. If we wanted to match against these arms, we could do the following:

```rust
match msg {
  Message::Quit => quit(),
  Message::Move {x, y} => move_point(x, y),
  Message::Write(text) => write(text),
  Message::ChangeColor(r, g, b) => update_color(r, g, b)
}
```

For our needs, we want to go on step further, destructuring the input. Let's change things a little:
```rust
struct Book { author: String, title: String, info: String}
enum Message {
  WriteBook(Book),
}
``````rust
match msg {
  Message::WriteBook(Book { author, title, .. }) => write_book(author, title),
}
```
See how the `author` and `title` parameters are made available, and how we're ignoring the `info` parameter with the `..` operator? We'll do something similar with `ReceiveNft`.

# Exercise

1. Set up the `ReceiveNft` variant on the first line, with its open parenthesis.
2. Add the `cw721::Cw721ReceiveMsg` struct; you'll make use of the `sender`, and `token_id` fields, but ignore the binary `msg` parameter.
3. Add the closing parenthesis and fat arrow to the function `receive_visa`, with all of the used parameters (five total for this variant). Don't forget the terminating comma!

> ### Formatting
> We formatted this variant for the ease of describing it; the Rust formatter will re-arrange it to its own standards, so you might see it differently in the rest of the lesson.

# Starter
```rust
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, info),
        ExecuteMsg::AssignVisa { msg } => assign_visa(msg, deps, info),
        // add the receive nft variant here (three lines)
    }
}
```

# Answer
```rust
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::ExecuteMsg;

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, info),
        ExecuteMsg::AssignVisa { msg } => assign_visa(msg, deps, info),
        ExecuteMsg::ReceiveNft(
          cw721::Cw721ReceiveMsg { sender, token_id, .. }
        ) => receive_visa(sender, token_id, deps, env, info),
    }
}
```