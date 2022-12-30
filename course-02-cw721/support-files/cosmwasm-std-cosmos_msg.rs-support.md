---
main file: cosmos_msg.rs
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
/** 
 * This is a truncated version of the actual file. More details on WasmMsg and CosmosMsgs in general can be found here: https://docs.rs/cosmwasm-std/latest/cosmwasm_std/enum.CosmosMsg.html#.
 */

pub enum CosmosMsg<T = Empty> {
    Bank(BankMsg),
    Custom(T),
    Staking(StakingMsg),
    Distribution(DistributionMsg),
    Stargate { type_url: String, value: Binary, },
    Ibc(IbcMsg),
    Wasm(WasmMsg),
    Gov(GovMsg),
}

pub enum WasmMsg {
    /// Dispatches a call to another contract at a known address (with known ABI).
    Execute {
        contract_addr: String,
        #[derivative(Debug(format_with = "binary_to_string"))]
        msg: Binary,
        funds: Vec<Coin>,
    },
    /// Instantiates a new contracts from previously uploaded Wasm code.
    Instantiate {
        admin: Option<String>,
        code_id: u64,
        #[derivative(Debug(format_with = "binary_to_string"))]
        msg: Binary,
        funds: Vec<Coin>,
        label: String,
    },
    /// Migrates a given contracts to use new wasm code. Passes a MigrateMsg to allow us to customize behavior.
    Migrate {
        contract_addr: String,
        new_code_id: u64,
        #[derivative(Debug(format_with = "binary_to_string"))]
        msg: Binary,
    },
    /// Sets a new admin (for migrate) on the given contract.
    /// Fails if this contract is not currently admin of the target contract.
    UpdateAdmin {
        contract_addr: String,
        admin: String,
    },
    /// Clears the admin on the given contract, so no more migration possible.
    /// Fails if this contract is not currently admin of the target contract.
    ClearAdmin { contract_addr: String },
}
```