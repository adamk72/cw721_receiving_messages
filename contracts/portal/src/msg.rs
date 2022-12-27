use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use universe::species::{SapienceScale, Sapient, Traveler};

#[cw_serde]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[cw_serde]
pub struct AssignVisaMsg {
    pub details: VisaAdminDetails,
}

#[cw_serde]
pub struct VisaAdminDetails {
    pub ape: Addr,
    /// The previous contract this is being sent from.
    pub contract: Addr,
    pub holder: Addr,
    /// The token_id of the Visa to be approved later.
    pub token_id: String,
}

#[cw_serde]
pub struct Visa {
    pub approved: bool,
    pub details: VisaAdminDetails,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
    JumpRingTravel { to: Addr },
    ReceiveNft(cw721::Cw721ReceiveMsg),
    AssignVisa { msg: AssignVisaMsg },
}

#[cw_serde]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpRingCheckResponse {
    pub valid: bool,
}
