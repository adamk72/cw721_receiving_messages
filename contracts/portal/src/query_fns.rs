use crate::{msg::JumpRingCheckResponse, state::CONFIG};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};
use cw721_visa::metadata::VisaMetadata;
use universe::species::{SapienceResponse, Traveler};

#[cw_serde]
pub struct VisasResponse {
    pub visas: Vec<VisaMetadata>,
}

pub fn minimum_sapience(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let out = to_binary(&SapienceResponse {
        level: config.minimum_sapience,
    })?;
    Ok(out)
}

pub fn jump_ring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
    Ok(out)
}
