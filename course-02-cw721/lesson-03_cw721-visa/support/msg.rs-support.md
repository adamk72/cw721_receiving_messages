---
main file: one file name
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub apes: Vec<Addr>,
    pub jump_ring: Addr,
    pub name: String,
    pub symbol: String,
}
```