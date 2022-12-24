use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
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
    pub account: Option<Addr>,
    pub attributes: Option<Vec<Trait>>,
    pub dna: Option<String>,
    pub image: Option<String>,
    pub name: Option<String>,
    pub origin: Option<Addr>,
    pub species: Option<Species>,
}
