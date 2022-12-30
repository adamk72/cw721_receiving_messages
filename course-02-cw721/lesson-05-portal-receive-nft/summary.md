---
main file: contract.rs
supporting files: all
---

# Overview
## Summary
Congratulations! Not only did you finish this lesson, but also this course.

In this lesson, we touched on quite a bit. We created two functions that could be called from other contracts, `AssignVisa` and `ReceiveNft`, the latter of which had us delving into the intimate details of `Cw721ReceiveMsg`. 

In general, we learned about:
- The specific Rust `enum` variants.
- More on how messaging occurs between contracts.
- More on `deps.querier.query`, first introduced in the *Intro to CosmWasm* course.

From the `cosmwasm` libraries, we touched on:
- The `save()` and `load()` functions from `cosmwasm_std Map`. 
- How `ReceiverExecuteMsg::ReceiveNft()` from the `cw721` package comes into play.
- The `WasmQuery` and `QueryRequest` enums. 
- How to write out a `WasmQuery::Smart` message.

## Next Up
Get on with the next course!

<!-- This should be the contract file -->
# Answer
```rust

```