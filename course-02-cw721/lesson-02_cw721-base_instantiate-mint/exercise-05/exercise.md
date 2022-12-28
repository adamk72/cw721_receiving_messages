---
main file: execute.rs
supporting files: crate/state.rs(TokenInfo), eth/erc721metadata.json
cosmwasm topics:
rust topics:
---

# Overview
> *"To mint or not to mint, that is the question!"* ~unknown Exodite economist

# MintMsg
The `execute` entry point for the `mint` function in the `cw721-base` contract as the following signature:
```rust
ExecuteMsg::Mint(msg) => self.mint(deps, env, info, msg),
```
It takes `msg` variable of type `MintMsg`, which is what we'll construct first. It is almost identical to the `TokenInfo` structure, as we'll be using the information from `MintMsg` to populate the a `TokenInfo` struct before saving it to our `IndexedMap` list of `tokens`.

## Token Id
Each NFT token needs to be able to be uniquely identified within the contract. This is done through a String type identification field.

The `token_id` member of `MintMsg` needs to be unique, otherwise it will be rejected. In the case of the `cw721-base` contract, they elected to require the minter entity to check for uniqueness. You could as easily write a contract that establishes its own list of unique ids at the time of mint rather than relying on outside input. 

## Token URI
In the `cw721-base` contract, it says about the `token_uri` field that it "should point to a JSON file that conforms to the ERC721 metadata JSON schema". This is required if you wish you contract to be compatible with other ERC721 compliant contracts. Since part of Cosmos' mandate is to act as the "Internet of Blockchains," the Cosmos community is following the standards with the largest deployer of NFT contracts, Ethereum, and their [EIP-721](https://eips.ethereum.org/EIPS/eip-721)<ExternalLink> standard. You can see the details of the schema in the `erc721metadata.json` tab.

Note, it's a "should" requirement (the EIP-721 standard says "may", actually), so you can go ahead and write your own custom metadata that supports your system's needs.


# Exercise

1. Add a `token_id` member of type `Addr`.
2. Add an `owner` member of type `Addr`.
3. The `token_uri` field is is of an optional `String` type.
4. The `extension` member of type `T`.

# Starter
```rust
use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct MintMsg<T> {
    pub // add id
    pub // add owner 
    pub // add uri 
    pub // add extension 
}
```

# Answer
```rust
use cosmwasm_schema::cw_serde;
#[cw_serde]
pub struct MintMsg<T> {
    pub token_id: String,
    pub owner: String,
    pub token_uri: Option<String>,
    pub extension: T,
}
```