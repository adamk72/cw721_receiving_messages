use cosmwasm_schema::cw_serde;

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
    pub image: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub origin: Option<String>,
}
