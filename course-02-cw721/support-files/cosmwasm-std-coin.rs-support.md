---
main file: coins.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::math::Uint128;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct Coin {
    pub denom: String,
    pub amount: Uint128,
}

impl Coin {
    pub fn new(amount: u128, denom: impl Into<String>) -> Self {
        Coin {
            amount: Uint128::new(amount),
            denom: denom.into(),
        }
    }
}
```