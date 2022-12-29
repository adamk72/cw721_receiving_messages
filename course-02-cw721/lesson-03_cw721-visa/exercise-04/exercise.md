---
main file: metadata.rs
supporting files: opensea-metadata-standards.rs, , universe-species.rs-support
cosmwasm topics: 
rust topics: Default
---

# Overview
> 

# Deviating from the Standards
In our little universe, we're not going to follow the OpenSea standards (well, just a little). We're going to create our on metadata structure called `VisaMetadata`. The point of doing so is to demonstrate that metadata is indeed arbitrary.

# Revisiting Rust Default
This may be course on CosmWasm, but we want to help reinforce your Rust skills as well. We've touched on the `Default` trait a few exercises ago when referring to this: `Cw721Contract::default().instantiate`.

You'll see in the exercise code boiler plate that we added the following attribute:
```rust
#[derive(Default)]
```
Why? Mostly for convenience, so that the metadata can be initialized without necessarily having to fill in every field. It allows you to do something like this:
```rust
extension: Some(Metadata {
                description: Some("Starship with quantum drive array".into()),
                name: Some("USS Orville (ECV-197)".to_string()),
                ..Metadata::default() // the reminder of the fields will call their default() function.
            })
```

For this contract, we updated the `Species` struct from the `universe` package to support the `Default` trait. Here is what it looks like (see the `species.rs` tab for more context):
```rust
impl Default for Species {
    fn default() -> Self {
        Species { name: "Unnamed".to_string(), sapience_level: SapienceScale::None, }
    }
}
```

An important takeaway here is you could write the metadata code without requiring `Default`. You would need to make sure that all of your fields are properly filled in ahead of time. However, with metadata, sometimes its best to provide a fallback or nothing since the data is often optional. 

# Exercise
Following our `Trait` work, we'll now add `VisaMetadata`. There are two notable members of `VisaMetadata`, `attribute` and `species`.

1. There several entries to add, all but two are of type `String`. We've filled those in already as boilerplate.
2. `attributes` is a vector of `Trait`s.
3. `species` is of type `Species`.

As noted in the overview, `Species` required `Default` to be implemented to work in this context. 

# Starter
```rust
use cosmwasm_schema::cw_serde;
use universe::species::Species;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
#[derive(Default)]
pub struct VisaMetadata {
    pub account: String,
    // attributes goes here
    pub dna: String,
    pub image: String,
    pub name: String,
    pub origin: String,
    // species goes here
}
// 
```

# Answer
```rust
use cosmwasm_schema::cw_serde;
use universe::species::Species;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
#[derive(Default)]
pub struct VisaMetadata {
    pub account: String,
    pub attributes: Vec<Trait>,
    pub dna: String,
    pub image: String,
    pub name: String,
    pub origin: String,
    pub species: Species,
}
```