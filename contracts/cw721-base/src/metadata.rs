use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use universe::species::SapienceScale;

#[derive(Serialize, Deserialize)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>, // URL to (preferably to decentralized) media.

    // from NEAR standard; will see if all are desired/needed
    pub issued_at: Option<String>, // ISO 8601 datetime when token was issued or minted
    pub expires_at: Option<String>, // ISO 8601 datetime when token expires
    pub starts_at: Option<String>, // ISO 8601 datetime when token starts being valid
    pub updated_at: Option<String>, // ISO 8601 datetime when token was last updated

    pub dna: Option<String>,
    pub species: Option<String>,
    pub sapience: Option<SapienceScale>,
    pub origin: Option<Addr>,   // Planet address
    pub identity: Option<Addr>, // The owner's wallet address
}

/* For later usage, maybe */
pub struct Attribute {
    trait_type: String,
    value: String,
}
/// Following: https://docs.opensea.io/docs/metadata-standards
pub struct OpenMetadata {
    pub description: Option<String>, // A human readable description of the item. Markdown is supported.
    pub external_url: Option<String>, // This is the URL that will appear below the asset's image on OpenSea and will allow users to leave OpenSea and view the item on your site.
    pub name: Option<String>,         // Name of the item.
    pub image: Option<String>, // This is the URL to the image of the item. Can be just about any type of image (including SVGs, which will be cached into PNGs by OpenSea), and can be IPFS URLs or paths. We recommend using a 350 x 350 image.
    pub attributes: Vec<Attribute>,
}
