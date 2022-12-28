---
main file: state.rs 
supporting files: cw721/query.rs, cw-utils/expiration.md,  
cosmwasm topics:
rust topics:
---

# Overview
> "So many visas to approve, so little time. Perfect opportunity for a lunch break."

# Tokens
The last element we're going to add to our `Cw721Contract` is the very reason we're creating a contract to start with: the list of tokens that the contract will be responsible for.

The `tokens` member of the structs is fairly complex in the `cw721-base` contract implementation. We don't necessarily need to know all of the underlying functionality to its core, so we'll just touch on the highlights here and in the next exercise.

# TokenInfo
When an NFT token is created (i.e., "minted"), there are customary fields that all NFTs, regardless of blockchain technology are expected to have: the `owner` and the optional URI. In addition to those, the CW721 spec requires support for an `Approve` function to allow granting permission to send or transfer an NFT to another entity.

Lastly, and optionally, the `cw721-base` contract allows for the extension of additional token specific metadata of any type.

# Approval
The `Approval` struct is required to track who is approved for sending and transferring NFTs. We'll need it for the Exercise section.
```rust
pub struct Approval {
    pub spender: Addr,
    pub expires: Expiration,
}
// see the approval-support.rs tab for the actual implementation features.
```
In this contract, the `approvals` member is simply a vector (`Vec<Approval>`) and is cleared out when a transfer or send happens.

# Exercise
We've set up the basis structure for you, taking in a type `T` generic for use with the `extension` member.

1. Add an `owner` member of type `Addr`.
2. Add an `approvals` vector of type `Approval`. 
n `owner` member of type `Addr`.
3. The `token_uri` field is is of an optional `String` type.
4. The `extension` member, while optional is far as using it is concerned, is required here and is simply of type `T`. 

As noted, the `extension` field doesn't have to be used, but a type must still be defined for it. As such, the `cw721-base` contract defines a default type that is use elsewhere:
```rust
pub struct Empty {}
type Extension = Option<Empty>;
``` 

# Starter
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    pub // add owner member here
    pub // add approvals vector here
    pub // add uri here 
    pub // add extension here
}
```

# Answer
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    pub owner: Addr,
    pub approvals: Vec<Approval>,
    pub token_uri: Option<String>,
    pub extension: T,
}
```