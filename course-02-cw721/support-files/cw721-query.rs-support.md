---
main file: one file name
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
/**
 * Full file can be found with the cw721 package at https://github.com/CosmWasm/cw-nfts/tree/main/packages/cw721.
 */

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}


```