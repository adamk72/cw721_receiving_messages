use cosmwasm_schema::cw_serde;
// use cosmwasm_std::Addr;
// use universe::species::SapienceScale;

// pub struct TokenMetadata {
//     pub title: Option<String>,
//     pub description: Option<String>,
//     pub media: Option<String>, // URL to (preferably to decentralized) media.

//     // from NEAR standard; will see if all are desired/needed
//     pub issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
//     pub expires_at: Option<String>, // ISO 8601 datetime when token expires
//     pub starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
//     pub updated_at: Option<String>, // ISO 8601 datetime when token was last updated

//     pub dna: Option<String>,
//     pub species: Option<String>,
//     pub sapience: Option<SapienceScale>,
//     pub origin: Option<Addr>,   // Planet address
//     pub identity: Option<Addr>, // The owner's wallet address
// }

/* For later usage, maybe */
#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct OpenMetadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}
