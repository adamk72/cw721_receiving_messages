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
