---
main file: cw721-base/msg.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

```