---
main file: Quiz
supporting files:
cosmwasm topics: CW721
rust topics:
general topics: NFTs
---

# Overview
> The Exodites are known for two things: their beautiful and ecologically diverse worlds and their love of bureaucratic processes.

# NFTs for CosmWasm
NFTs &mdash; non-fungible tokens &mdash; for CosmWasm are defined by the CW721 specification. The spec tells us what should be the minimum functionality for supporting NFTs on a Cosmos blockchain.

What we'll be creating over this course is a contract that generates NFTs. It's important to note that the contract is, by itself, *not* an NFT. Rather, it is the generator of NFTs. Through a function that is typically called `Mint`, the contract creates tokens that represent the NFTs. Thus, an NFT contract is actually a list of NFTs, each of which could be said to have an an owner (the person who paid for the NFT, typically).

## CW721 Specification Details
The CW721 spec is broken into four sections, each of which we'll be exploring over this lesson and the broader course.

- Messages, which are the executable functions for the contract and include `TransferNft`, `SendNft`, `Approve`, and `Revoke`.
- Queries, which are for making inquires to the contract and include `OwnerOf`, `Approval`, `AllOperators` and `NumTokens`.
- Receiver is special; it is not a requirement to write a CW721 contract per se, but rather if you would like *another* contract to take ownership of an NFT via the `SendNft` message. It includes the single function of `ReceiveNft`. 
- Metadata, which includes additional queries specifically for the contract details. These queries are `ContractInfo`, `NftInfo`, and `AllNftInfo`. 

> ### Metadata
> "Metadata" is especially meta in an NFT contract. There is the metadata that is specific to the contract itself, but also an optional metadata extension that we will be using to describe the individual tokens.

Interestingly enough, you may have noticed that `Mint` is not part of the CW721 spec. This because the act of NFT creation is very specific to the type of NFT and how the contract owner might choose to share and distribute NFTs. There is no standard for NFT generation in that sense.


# Exercise
Quiz time! Simply place an `x` in the checkbox that is your answer.

# Starter
```markdown
Which of these is *not* in the CW721 Specification?
[] `SendNft`
[] `OwnerOf`
[] `Mint`
[] `ContractInfo`

```

# Answer
```markdown
Which of these is *not* in the CW721 Specification?
[] `SendNft`
[] `OwnerOf`
[x] `Mint`
[] `ContractInfo`
```