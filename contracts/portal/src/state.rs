use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw721_visa::metadata::VisaMetadata;
use cw_storage_plus::Item;
use universe::species::{SapienceScale, Sapient};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
    pub visas: Option<Vec<VisaMetadata>>,
}

pub const CONFIG: Item<Config> = Item::new("config");
