---
main file: expiration.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Timestamp;

/// Expiration represents a point in time when some event happens.
/// It can compare with a BlockInfo and will return is_expired() == true
/// once the condition is hit (and for every block in the future)
#[cw_serde]
#[derive(Copy)]
pub enum Expiration {
    /// AtHeight will expire when `env.block.height` >= height
    AtHeight(u64),
    /// AtTime will expire when `env.block.time` >= time
    AtTime(Timestamp),
    /// Never will never expire. Used to express the empty variant
    Never {},
}
```