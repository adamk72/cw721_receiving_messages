---
main file: wasm.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Binary;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum WasmQuery {
    /// This queries the public API of another contract at a known address (with known ABI)
    /// Return value is whatever the contract returns (caller should know), wrapped in a ContractResult that is JSON encoded.
    Smart {
        contract_addr: String,
        msg: Binary,
    },
    /// This queries the raw kv-store of the contract.
    /// Returns the raw, unparsed data stored at that key, which may be an empty vector if not present
    Raw {
        contract_addr: String,
        key: Binary,
    },
    /// Returns a ContractInfoResponse with metadata on the contract from the runtime
    ContractInfo { contract_addr: String },
}
```