---
main file: mod.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryRequest<C> {
    Bank(BankQuery),
    Custom(C),
    Staking(StakingQuery),
    Stargate {
        path: String,
        data: Binary,
    },
    Ibc(IbcQuery),
    Wasm(WasmQuery),
}
```