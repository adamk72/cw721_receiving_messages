---
applies to: 
---

**This is a help file only**

# Answer
# More on `_transfer_nft`
If you're interested in the details for `_transfer_nft`, check out the source code on the CosmWasm GitHub repo for the `cw721-base` contract's [execute](https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/execute.rs) file. The `check_can_approve()` is another demonstration of how `Map` works (using the `operators` list). 

# More on the Actor Model
CosmWasm is based on the <ExternalLink href="https://docs.cosmwasm.com/docs/1.0/architecture/actor">Actor Model</ExternalLink> design pattern. In this pattern, Actors do not talk directly to one another (i.e., do not call functions directly) but rather send messages to one another. Here's a basic interface for the Actor model:

```rust
pub trait Actor {
  fn handle(msgPayload: &[u8]) -> Vec<Msg>;
}

pub struct Msg {
  pub destination: Vec<u8>,
  pub payload: Vec<u8>,
}
```