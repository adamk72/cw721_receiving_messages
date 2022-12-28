---
main file: one file name
supporting files: n/a
---

**This is a support file only**

# Answer
```rust
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw721-base";
const CONTRACT_VERSION: &str = "0.16.0";
pub fn instantiate(
    &self,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<C>> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let info = ContractInfoResponse {
      name: msg.name,
      symbol: msg.symbol,
    };
    self.contract_info.save(deps.storage, &info)?;

    let minter = deps.api.addr_validate(&msg.minter)?;
    self.minter.save(deps.storage, &minter)?;

    Ok(Response::default())
}
```