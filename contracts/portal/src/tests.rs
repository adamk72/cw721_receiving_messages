#![cfg(test)]

use crate::{contract::execute, msg::ExecuteMsg};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MOCK_CONTRACT_ADDR},
    to_binary, Response,
};
use cw721_base::spec::Cw721ReceiveMsg;

#[test]
pub fn receive_visa() {
    let mut deps = mock_dependencies();
    let token_id = "petrify".to_string();
    // Send a Cw721ReceiveMsg

    let msg = to_binary("You now have the melting power").unwrap();
    let payload = Cw721ReceiveMsg {
        sender: String::from("venus"),
        token_id: token_id.clone(),
        msg,
    };

    let sender_name = "venus";
    let sender = mock_info(sender_name, &[]);
    let rcv_msg = ExecuteMsg::ReceiveVisa { msg: payload };
    let res = execute(deps.as_mut(), mock_env(), sender, rcv_msg).unwrap();

    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "receive_visa")
            .add_attribute("new_owner", MOCK_CONTRACT_ADDR)
            .add_attribute("new_token_id", token_id)
    );
}

mod initiate_jump_ring_travel {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info},
        Addr, Response,
    };

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
        VISAS
            .save(
                &mut deps.storage,
                (&info.sender, &info.sender.clone()),
                &visa,
            )
            .unwrap();
        let res = initiate_jump_ring_travel(dest, deps.as_mut(), info.clone()).unwrap();
        assert_eq!(
            res,
            Response::new()
                .add_attribute("action", "initiate_jump_ring_travel")
                .add_attribute("traveler", &info.sender)
        )
    }
}
