use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::execute_fns::{
    initiate_jump_ring_travel, receive_visa, set_minimum_sapience, set_planet_name,
    set_sapient_names,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query_fns::{jump_ring_check, minimum_sapience};
use crate::state::{config, State};

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::JumpRingPreCheck { traveler } => jump_ring_check(traveler),
        QueryMsg::MinimumSapience {} => minimum_sapience(deps),
    }
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetPlanetName { to } => set_planet_name(to, deps, info),
        ExecuteMsg::SetSapientNames { to } => set_sapient_names(to, deps, info),
        ExecuteMsg::SetMinimumSapience { to } => set_minimum_sapience(to, deps, info),
        ExecuteMsg::JumpRingTravel { to } => initiate_jump_ring_travel(to, deps, info),
        ExecuteMsg::ReceiveVisa { msg } => receive_visa(msg, deps, env, info),
    }
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender,
        planet_name: msg.planet_name,
        planet_sapients: msg.planet_sapients,
        minimum_sapience: msg.minimum_sapience,
    };
    config(deps.storage).save(&state)?;
    Ok(Response::new()
        .add_attribute("owner", state.owner)
        .add_attribute("minimum_sapience", state.minimum_sapience.as_str()))
}