---
main file: cw2/lib.rs 
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
/**
 * The full code for the CW2 spec can be found here: https://github.com/CosmWasm/cw-plus/tree/main/packages/cw2.
 */

use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

/* `ContractInfo` must be stored under the `"contract_info"` key which translates
to `"636F6E74726163745F696E666F"` in hex format.
Since the state is well defined, we do not need to support any "smart queries".
We do provide a helper to construct a "raw query" to read the ContractInfo
of any CW2-compliant contract. */
pub const CONTRACT: Item<ContractVersion> = Item::new("contract_info");

#[cw_serde]
pub struct ContractVersion {
    /// contract is the crate name of the implementing contract, eg. `crate:cw20-base`
    /// we will use other prefixes for other languages, and their standard global namespacing
    pub contract: String,
    /// version is any string that this implementation knows. It may be simple counter "1", "2".
    /// or semantic version on release tags "v0.7.0", or some custom feature flag list.
    /// the only code that needs to understand the version parsing is code that knows how to
    /// migrate from the given contract (and is tied to it's implementation somehow)
    pub version: String,
}

/// set_contract_version should be used in instantiate to store the original version, and after a successful
/// migrate to update it
pub fn set_contract_version<T: Into<String>, U: Into<String>>(
    store: &mut dyn Storage,
    name: T,
    version: U,
) -> StdResult<()> {
    let val = ContractVersion {
        contract: name.into(),
        version: version.into(),
    };
    CONTRACT.save(store, &val)
}
```