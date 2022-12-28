---
main file: contract.rs 
supporting files: comma separated list
cosmwasm topics: Deps, Api, addr_validate
rust topics:
---

# Overview
> 

# Minter
Another member of `Cw721Contract` struct was `minter`:
```rust
pub struct Cw721Contract<'a>
{
    pub minter: Item<'a, Addr>,
    // ... other members
}
```

Notice that `minter` is another `Item` object, so we'll follow the same progression as we did last lesson. It takes a struct type of `Addr`, which should remind use that this may be an account or a contract. It is this entity that will have sole permission to create the NFT tokens. As noted before, the idea of "minting" is not actually a requirement of the CW721 spec, so this code might be customized for your own use. However, for this exercise, we'll keep with what is in the `cw721-base` contract.

# Validating Addresses
It's critical that we confirm the nature of any addressed passed to our contracts. If the `minter` address is not at the very least a properly formed string, then we'll have instantiated a contract that we can't use. The `Deps` type supports the `Api` [trait](https://docs.rs/cosmwasm-std/latest/cosmwasm_std/trait.Api.html)<ExternalLink> from `cosmwasm-std` which can help us out here.
```rust
// Simplified
pub struct Deps {
    pub storage: &dyn Storage,
    pub api: &dyn Api,
    pub querier: QuerierWrapper,
}

// See the full trait in cosmwasm-std/traits.rs tab for more details.
pub trait Api {
    fn addr_validate(&self, human: &str) -> StdResult<Addr>;
    fn addr_canonicalize(&self, human: &str) -> StdResult<CanonicalAddr>;
    fn addr_humanize(&self, canonical: &CanonicalAddr) -> StdResult<Addr>;
    // ...more
}
```

In particular, we'll be using the `addr_validate` function which will verify that any passed address is properly normalized. 
```rust
let input = "i_am_probably_invalid";
let validated: Addr = api.addr_validate(input).unwrap();
assert_eq!(validated, input); // fail!
```

# Exercise

1. Create a variable called 'minter'; validate the `msg.minter` string.
2. Save the results of `&minter` to the `self.minter` `Item` field.

In both cases, you'll need to remember to apply `?`. 

And that's it for the `instantiate` function! Next, we'll look into how minting works.

# Starter
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

    // validate the minter
    // save the minter

    Ok(Response::default())
}
```

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