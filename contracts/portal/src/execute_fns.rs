use crate::error::ContractError;
use crate::state::config;
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
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.minimum_sapience = to;
    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    state.planet_name = to.clone();
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    state.planet_sapients = to;
    config(deps.storage).save(&state)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}
