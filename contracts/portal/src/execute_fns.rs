use crate::{error::ContractError, state::CONFIG};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw721_base::spec::Cw721ReceiveMsg;
use universe::species::{SapienceScale, Sapient};

pub fn receive_visa(
    msg: Cw721ReceiveMsg,
    _deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", msg.token_id))
}

pub fn initiate_jump_ring_travel(
    _to: Addr,
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.minimum_sapience = to;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.planet_name = to.clone();
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.planet_sapients = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}
