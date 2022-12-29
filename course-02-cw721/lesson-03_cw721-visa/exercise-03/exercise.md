---
main file: metadata.rs 
supporting files: opensea-metadata-standards.rs
cosmwasm topics: NFT::metadata
rust topics: trait (mention only)
---

# Overview
> The trouble with travel between Exodite eco-worlds is that you need a new visa for each Jump Ring. Guess I'll get in queue... *again*. I hear the Mel'Mirrys Asteroid of Sargas 4 is breathtaking to see. 
@todo: change or move this one; not quite right here.

# Metadata
What is metadata? You hear about it all the time, not just for NFTs, but also when it comes other software and services, like payment systems, search engines, and financial apps... most any type of software can have metadata associated with it. Metadata is data about other data.

In the NFT contract world, there are two types of metadata commonly referred to:
1. The metadata of the contract itself. Things like the contract's name and symbol are included in that list.
2. The metadata of a specific NFT token. In the case of the old standby in NFTs, CryptoKitties, the metadata stores data on the kitty's attributes, like fur color, tail length, and other features of the image.

## Standards
As you might imagine, data about data might be most useful if it is well understood and organized. This is where standards come in. If you're developing your own personal NFT system, you can make the metadata be anything you want it to be. However, if you want to ensure that your NFTs work on other systems (within the Cosmos Network or elsewhere), you'll need to follow some standards. The OpenSea metadata [standard](https://docs.opensea.io/docs/metadata-standards) is the defacto standard for NFTs across multiple blockchain networks; follow their recommendations for the best possible compatibility.

## Attributes and Traits
From the OpenSea standard comes the idea of 'traits' (not to be confused with the Rust `trait` keyword). Traits are a very generic way to define arbitrary types of data, from strings to numbers to dates. For example:
```json
{
  "display_type": "date", 
  "trait_type": "birthday", 
  "value": 1546360800
}
```
The system that ingests any traits needs to be abel to covert the JSON input into something more user friendly; the standard assures that the data will be consistent.

Traits are part of a larger `attributes` structure. Though we won't be using attributes and traits in a meaningful way in this lesson, we'll set the groundwork for later use here.


# Exercise
Even though `trait` is defined by the OpenSea standard, there isn't currently a standard package to use for CosmWasm, so we'll have to define it ourselves (long with `attributes` in the next lesson).

1. Create a `Trait` struct.
2. Populate with the same fields as from the above Date example, where:
    - `display_type` is an optional `String`.
    - `trait_type` and `value` are both `String`s.

# Starter
```rust
use cosmwasm_schema::cw_serde;

#[cw_serde]
// add the trait struct here (five lines)
```

# Answer
```rust
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
```