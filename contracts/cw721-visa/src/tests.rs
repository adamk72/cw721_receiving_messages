#![cfg(test)]

use crate::{
    contract::{instantiate, Cw721VisaContract, ExecuteMsg},
    metadata::VisaMetadata,
    msg::InstantiateMsg,
};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Response,
};
use cw2::{query_contract_info, ContractVersion};
use cw721::Cw721Query;
use cw721_base::{MintMsg, MinterResponse};
use cw_multi_test::{App, ContractWrapper, Executor};

fn contract_code() -> Box<dyn cw_multi_test::Contract<cosmwasm_std::Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    // .with_reply(cw721_visa::contract::reply);

    Box::new(contract)
}

const CREATOR: &str = "creator";

#[test]
fn check_contract_version() {
    let mut app = App::default();
    let sender = Addr::unchecked("owner");
    let code_id = app.store_code(contract_code());

    let contract_addr = app
        .instantiate_contract(
            code_id,
            sender,
            &InstantiateMsg {
                name: "SpaceShips".to_string(),
                symbol: "SPACE".to_string(),
                apes: vec![Addr::unchecked(CREATOR), Addr::unchecked("venus")],
                jump_ring: Addr::unchecked("venus"),
            },
            &[],
            "echo".to_string(),
            None,
        )
        .unwrap();

    let version: ContractVersion = query_contract_info(&app, contract_addr).unwrap();

    assert_eq!(
        ContractVersion {
            contract: "cw721-visa".to_string(),
            version: "0.16.0".to_string(),
        },
        version
    );
}
#[test]
fn use_metadata_extension() {
    let mut deps = mock_dependencies();
    let contract = Cw721VisaContract::default();

    let info = mock_info(CREATOR, &[]);
    let init_msg = InstantiateMsg {
        name: "SpaceShips".to_string(),
        symbol: "SPACE".to_string(),
        apes: vec![Addr::unchecked(CREATOR), Addr::unchecked("venus")],
        jump_ring: Addr::unchecked("venus"),
    };

    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();
    assert_eq!(res, Response::new().add_attribute("minter", CREATOR));

    let token_id = "Enterprise";
    let mint_msg = MintMsg {
        token_id: token_id.to_string(),
        owner: "john".to_string(),
        token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
        extension: Some(VisaMetadata {
            name: Some("Starship USS Enterprise".to_string()),
            ..VisaMetadata::default()
        }),
    };
    let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
    contract
        .execute(deps.as_mut(), mock_env(), info, exec_msg)
        .unwrap();

    let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
    assert_eq!(res.token_uri, mint_msg.token_uri);
    assert_eq!(res.extension, mint_msg.extension);

    let res = contract.minter(deps.as_ref()).unwrap();
    assert_eq!(
        res,
        MinterResponse {
            minter: CREATOR.to_string()
        }
    )
}
