#![cfg(test)]
use crate::contract::{execute, instantiate, query};
use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::Empty;
use cosmwasm_std::{testing::mock_info, to_binary, Addr, Response};
use cw721_base::spec::Cw721ReceiveMsg;
use cw721_visa::contract::{
    execute as visa_execute, instantiate as visa_init, query as visa_query,
    ExecuteMsg as CW721ExecuteMsg,
};
use cw721_visa::msg::InstantiateMsg as CW721InstantiateMsg;
use cw_multi_test::{App, Contract, ContractWrapper};

fn mock_app() -> App {
    App::default()
}

fn cw721_visa_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(visa_execute, visa_init, visa_query);
    Box::new(contract)
}

fn portal_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

struct Names<'a> {
    visa_sym: &'a str,
    jr_owner: &'a str,
    visa_holder: &'a str,
    ape_minter: &'a str,
}
const NAMES: Names<'static> = Names {
    visa_sym: "VISA_SYM",
    jr_owner: "jr_owner",
    visa_holder: "holder",
    ape_minter: "ape_minter",
};

mod receive_visa {
    use cosmwasm_std::Event;
    use cw721_base::MintMsg;
    use cw721_visa::metadata::VisaMetadata;
    use cw_multi_test::Executor;
    use universe::species::{SapienceScale, Sapient};

    use super::*;
    use crate::msg::{ExecuteMsg, InstantiateMsg};

    #[test]
    pub fn can_receive() {
        let sender_name = "venus";
        let mut app = mock_app();
        let jump_ring_box = portal_contract();
        let cw721_visa_box = cw721_visa_contract();
        let p_code = app.store_code(jump_ring_box);
        let v_code = app.store_code(cw721_visa_box);

        let jr_init_msg = InstantiateMsg {
            planet_name: "foobar".to_string(),
            planet_sapients: vec![Sapient {}],
            minimum_sapience: SapienceScale::Medium,
        };

        let jump_ring_contract = app.instantiate_contract(
            p_code,
            Addr::unchecked(NAMES.jr_owner),
            &jr_init_msg,
            &[],
            NAMES.jr_owner,
            None,
        );

        let jr_addr = jump_ring_contract.unwrap();
        let visa_init_msg = CW721InstantiateMsg {
            apes: vec![Addr::unchecked(NAMES.ape_minter)],
            name: NAMES.visa_holder.to_string(),
            symbol: NAMES.visa_sym.to_string(),
            jump_ring: jr_addr.clone(),
        };

        let visa_contract = app.instantiate_contract(
            v_code,
            Addr::unchecked(NAMES.ape_minter),
            &visa_init_msg,
            &[],
            NAMES.ape_minter,
            None,
        );

        /***** Actual testing *****/
        // First, mint a token called "dakkadakka"
        let token_id = "dakkadakka".to_string();
        let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

        let mint_msg: CW721ExecuteMsg = CW721ExecuteMsg::Mint(MintMsg::<Option<VisaMetadata>> {
            token_id: Some(token_id.clone()),
            owner: String::from(NAMES.ape_minter),
            token_uri: Some(token_uri),
            extension: Some(VisaMetadata {
                name: Some(NAMES.visa_holder.to_string()),
                image: None,
                attributes: None,
                origin: None,
            }),
        });

        let visa_addr = visa_contract.unwrap();
        app.execute_contract(
            Addr::unchecked(NAMES.ape_minter),
            visa_addr.clone(),
            &mint_msg,
            &[],
        )
        .unwrap();

        // Send a Cw721ReceiveMsg
        let msg = to_binary("You now have the melting power").unwrap();
        let payload = Cw721ReceiveMsg {
            sender: sender_name.to_string(),
            token_id: token_id.clone(),
            msg,
        };

        let rcv_msg = ExecuteMsg::ReceiveVisa { msg: payload };
        let res = app
            .execute_contract(visa_addr.clone(), jr_addr.clone(), &rcv_msg, &[])
            .unwrap();

        res.assert_event(&Event::new("wasm").add_attribute("action", "receive_visa"));
        res.assert_event(&Event::new("wasm").add_attribute("new_owner", jr_addr.to_string()));
        res.assert_event(&Event::new("wasm").add_attribute("new_token_id", token_id.clone()));
    }
}

mod initiate_jump_ring_travel {
    use super::*;
    use crate::{
        error::ContractError,
        execute_fns::initiate_jump_ring_travel,
        msg::{Visa, VisaDetails},
        state::VISAS,
    };

    #[test]
    pub fn visa_not_on_list() {
        let mut deps = mock_dependencies();
        let sender_name = "not on list";
        let sender = mock_info(sender_name, &[]);
        let dest = Addr::unchecked("mars");

        let err = initiate_jump_ring_travel(dest, deps.as_mut(), sender).unwrap_err();
        assert_eq!(err, ContractError::NotOnList {})
    }

    #[test]
    pub fn visa_is_approved() {
        let mut deps = mock_dependencies();
        let info = mock_info("zeus", &[]);
        let dest = Addr::unchecked("mars");

        let details = VisaDetails {
            ape: Addr::unchecked("ape"),
            contract: Addr::unchecked("mars"),
            holder: Addr::unchecked("mars"),
            token_id: "dakkadakka".to_string(),
        };
        let visa = Visa {
            approved: true,
            details: details,
        };
        VISAS.save(&mut deps.storage, &info.sender, &visa).unwrap();
        let res = initiate_jump_ring_travel(dest, deps.as_mut(), info.clone()).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("action", "initiate_jump_ring_travel")
                .add_attribute("traveler", &info.sender)
        )
    }
}
