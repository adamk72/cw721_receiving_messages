use crate::{
    error::ContractError,
    msg::{AssignVisaMsg, Visa},
    query_fns::minimum_sapience,
    state::{CONFIG, VISAS},
};
use cosmwasm_std::{
    from_binary, to_binary, Addr, DepsMut, Env, MessageInfo, QueryRequest, Response, WasmQuery,
};
use cw721::{Cw721QueryMsg, NftInfoResponse};
use cw721_visa::metadata::VisaMetadata;
use universe::species::{SapienceResponse, SapienceScale, Sapient};

pub fn receive_visa(
    sender: String,
    token_id: String,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let query = WasmQuery::Smart {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&Cw721QueryMsg::NftInfo {
            token_id: token_id.clone(),
        })?,
    };

    let res: NftInfoResponse<VisaMetadata> = deps.querier.query(&QueryRequest::Wasm(query))?;
    let incoming_sapience = res.clone().extension.species.sapience_level;
    let contract_min_sapience: SapienceResponse =
        from_binary(&minimum_sapience(deps.as_ref()).unwrap()).unwrap();
    if incoming_sapience.as_value() < contract_min_sapience.level.as_value() {
        return Err(ContractError::NotSmartEnough {});
    }

    VISAS.update(deps.storage, &Addr::unchecked(sender), |op| match op {
        None => Err(ContractError::NotOnList {}),
        Some(mut visa) => {
            visa.approved = true;
            Ok(visa)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", token_id))
}

/// Receive initial details and add to visa whitelist for later verification.
pub fn assign_visa(
    msg: AssignVisaMsg,
    deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // The visa will be approved once the the nft is sent over.

    let visa = Visa {
        approved: false,
        details: msg.details.clone(),
    };

    VISAS.save(deps.storage, &msg.details.holder, &visa)?;

    Ok(Response::new().add_attribute("action", "assign_visa"))
}

pub fn initiate_jump_ring_travel(
    _to: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    /*  The visa has to be approved for this to happen.
        1. check that the sender is the visas list
        2. check that the visa is approved (which happens when they send their visa to this contract)
    */

    let visa = match VISAS.load(deps.storage, &info.sender) {
        Ok(v) => v,
        Err(_) => return Err(ContractError::NotOnList {}),
    };

    if !visa.approved {
        return Err(ContractError::Unapproved {});
    }

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", &info.sender))
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
