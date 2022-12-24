#![cfg(test)]

use cosmwasm_std::Event;
use cosmwasm_std::{to_binary, Addr};
use cw721_base::spec::Cw721ReceiveMsg;
use cw721_base::MintMsg;
use cw721_visa::contract::ExecuteMsg as CW721ExecuteMsg;
use cw721_visa::metadata::VisaMetadata;
use cw721_visa::msg::InstantiateMsg as CW721InstantiateMsg;
use cw_multi_test::Executor;
use integration::{
    consts::{APE_MINTER, APE_OWNER, JR_OWNER, VISA_HOLDER, VISA_SYM},
    contract_helpers::ContractBase,
    contract_mocks::{JumpRingContract, VisaContract},
    test_env::mock_app,
};
use portal::msg::{ExecuteMsg as PortalExecuteMsg, InstantiateMsg as PortalInitMsg};
use universe::species::{SapienceScale, Sapient};

#[test]
pub fn full_workflow() {
    let sender_name = "venus";
    let mut app = mock_app();
    let p_code = app.store_code(JumpRingContract::contract_code());
    let v_code = app.store_code(VisaContract::contract_code());

    let jr_init_msg = PortalInitMsg {
        planet_name: "foobar".to_string(),
        planet_sapients: vec![Sapient {}],
        minimum_sapience: SapienceScale::Medium,
    };

    let jump_ring_addr = app
        .instantiate_contract(
            p_code,
            Addr::unchecked(JR_OWNER),
            &jr_init_msg,
            &[],
            JR_OWNER,
            None,
        )
        .unwrap();

    let jump_ring_contract = JumpRingContract(jump_ring_addr);

    let visa_init_msg = CW721InstantiateMsg {
        apes: vec![Addr::unchecked(APE_MINTER), Addr::unchecked(APE_OWNER)],
        name: VISA_HOLDER.to_string(),
        symbol: VISA_SYM.to_string(),
        jump_ring: jump_ring_contract.addr(),
    };

    let visa_contract_addr = app
        .instantiate_contract(
            v_code,
            Addr::unchecked(APE_OWNER),
            &visa_init_msg,
            &[],
            APE_MINTER,
            None,
        )
        .unwrap();
    let visa_contract = VisaContract(visa_contract_addr.clone());

    /***** Actual testing *****/
    // First, mint a token called "dakkadakka"
    let token_id = "dakkadakka".to_string();
    let token_uri = "https://www.merriam-webster.com/dictionary/melt".to_string();

    let mint_msg: CW721ExecuteMsg = CW721ExecuteMsg::Mint(MintMsg::<Option<VisaMetadata>> {
        token_id: Some(token_id.clone()),
        owner: String::from(APE_OWNER),
        token_uri: Some(token_uri),
        extension: Some(VisaMetadata {
            name: Some(VISA_HOLDER.to_string()),
            ..VisaMetadata::default()
        }),
    });

    app.execute_contract(
        Addr::unchecked(APE_MINTER),
        visa_contract.addr(),
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

    let rcv_msg = PortalExecuteMsg::ReceiveVisa { msg: payload };
    let res = app
        .execute_contract(
            visa_contract.addr(),
            jump_ring_contract.addr(),
            &rcv_msg,
            &[],
        )
        .unwrap();

    println!("Result: {:?}", res);

    res.assert_event(&Event::new("wasm").add_attribute("action", "receive_visa"));
    res.assert_event(
        &Event::new("wasm").add_attribute("new_owner", jump_ring_contract.addr().to_string()),
    );
    res.assert_event(&Event::new("wasm").add_attribute("new_token_id", token_id.clone()));
}
