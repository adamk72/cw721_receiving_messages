use cosmwasm_std::Addr;
use cw721_base::spec::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use universe::species::{SapienceScale, Sapient, Traveler};

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
    JumpRingTravel { to: Addr },
    ReceiveVisa { msg: Cw721ReceiveMsg },
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct JumpRingCheckResponse {
    pub valid: bool,
}
