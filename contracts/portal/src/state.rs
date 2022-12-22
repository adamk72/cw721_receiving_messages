use crate::msg::Visa;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use universe::species::{SapienceScale, Sapient};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Stored as (visa_holder, last_contract_address)
pub const VISAS: Map<(&Addr, &Addr), Visa> = Map::new("visas");
