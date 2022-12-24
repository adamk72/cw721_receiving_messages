use crate::{
    error::ContractError,
    msg::{PreapproveVisaMsg, Visa},
    state::{CONFIG, VISAS},
};
use cosmwasm_std::{to_binary, Addr, DepsMut, Env, MessageInfo, QueryRequest, Response, WasmQuery};
use cw721_base::spec::{Cw721QueryMsg, Cw721ReceiveMsg, NftInfoResponse};
use cw721_visa::metadata::VisaMetadata;
use universe::species::{SapienceScale, Sapient};

pub fn receive_visa(
    msg: Cw721ReceiveMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // let sender = deps.api.addr_validate(&owner)?;
    // let visa = VISAS
    //     .prefix(&sender)
    //     .filter(|v| v == msg.token_id)
    //     .collect();
    /*  We need to check for the following:
        1. That the owner-sender is on some sort of approved list.
        2. That the token_id has been pre-vetted.
        3. That the token_id and the sender match on the approved list.
        4. That the token_id isn't already on the list.

        We need to:
        1. Check back with the cw721 contract to get the visa info based on token_id.
        2. Confirm the user is under the sapience value or not on the excluded list.
        3. Confirm the the user is not already on the visa list.
        4. Add to the VISAS list as preapproved.
    */

    let query = WasmQuery::Smart {
        contract_addr: info.sender.to_string(),
        msg: to_binary(&Cw721QueryMsg::NftInfo {
            token_id: msg.token_id.clone(),
        })?,
    };

    let res: NftInfoResponse<VisaMetadata> = deps.querier.query(&QueryRequest::Wasm(query))?;
    println!("NftInfoResponse {:?}", res);

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address)
        .add_attribute("new_token_id", msg.token_id))
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

/// Receive initial details and add to visa whitelist for later verification.
pub fn assign_visa(
    visa_msg: PreapproveVisaMsg,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // The visa will be approved once the the nft is sent over.

    let visa = Visa {
        approved: false,
        details: visa_msg.details,
    };

    VISAS.save(deps.storage, &info.sender, &visa)?;

    Ok(Response::new().add_attribute("action", "preapprove_visa"))
}
