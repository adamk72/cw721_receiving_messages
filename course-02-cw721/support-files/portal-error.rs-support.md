---
main file: portal/error.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Not Approved")]
    Unapproved {},

    #[error("You don't qualify to visit.")]
    NotSmartEnough {},

    #[error("Visa not on List")]
    NotOnList {},
}

```