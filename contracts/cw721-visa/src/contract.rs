use crate::{
    msg::InstantiateMsg,
    state::{Config, CONFIG},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::InstantiateMsg as Cw721BaseInstantiateMsg;
pub type Cw721NonTransferableContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

// Version info for migration
const CONTRACT_NAME: &str = "cw721-visa";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct VisaMetadata {
    pub image: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub origin: Option<String>,
}

pub type Extension = Option<VisaMetadata>;

pub type Cw721VisaContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw721_base::{ContractError, Cw721Contract};

#[entry_point]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
        .map_err(ContractError::Std)?;

    let config = Config {
        jump_ring: msg.jump_ring,
        apes: msg.apes.clone(),
    };

    let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        minter: msg.apes[0].to_string(), // First ape in list becomes minter
    };

    // @todo: other apes become "approves?"

    Cw721NonTransferableContract::default().instantiate(
        deps.branch(),
        env,
        info,
        cw721_base_instantiate_msg,
    )?;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("minter", msg.apes[0].to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Cw721VisaContract::default().execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Cw721VisaContract::default().query(deps, env, msg)
}
