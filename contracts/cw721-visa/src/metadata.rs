use cosmwasm_schema::cw_serde;
use universe::species::Species;

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
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
