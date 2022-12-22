use crate::species::Species;
use cosmwasm_std::Addr;

pub enum ExecuteMsg {
    Snitch {
        address: Addr,
        name: String,
        species: Species,
    },
}
