---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
This was a short lesson! It's purpose was to get you more familiar with not just the `cw721` spec code, but also serve as refresher from the *Intro to CosmWasm* course topic regarding `CosmosMsg`s and `WasmMsg::Execute` (plus a little more practice on using Rust `impl`). The big takeaway was learning a little more about how CosmWasm is part of the bigger Cosmos ecosystem as a module within the Cosmos SDK.

In general, we learned about:
- Details of the `ExecuteMsg::SendNft` and `send_nft` from the `cw721-base` package.
- `Cw721ReceiveMsg` and how it is implemented.
- A little about the Cosmos SDK.

From the `cosmwasm` libraries, we touched on:
- `Response`, the wrapper around CosmWasm's event system that is responsible for sending the aggregated message details.


## Next Up
We're going to take a trip to past and make some changes to the Jump Ring contract so we can learn about transferring NFTs between contracts!
<!-- This should be the contract file -->
# Answer
```rust

```