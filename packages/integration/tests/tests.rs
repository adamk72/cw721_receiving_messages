#![cfg(test)]

use cosmwasm_std::Event;
use cosmwasm_std::{to_binary, Addr};
use cw721::Cw721ReceiveMsg;
use cw721_base::MintMsg;
use cw721_visa::contract::ExecuteMsg as CW721ExecuteMsg;
use cw721_visa::metadata::VisaMetadata;
use cw721_visa::msg::InstantiateMsg as CW721InstantiateMsg;
use cw_multi_test::Executor;
use integration::consts::{
    APE_ADMIN, BINARY_RCV_MSG, DESTINATION, OTHER_ALIEN, TOKEN_ID_1, TOKEN_URI_1, VISA_HOLDER_ACCT,
};
use integration::test_env::get_species_by_level;
use integration::{
    consts::{APE_CONTRACT_OWNER, APE_MINTER, JR_OWNER, VISA_HOLDER_NAME, VISA_SYM},
    contract_helpers::ContractBase,
    contract_mocks::{JumpRingContract, VisaContract},
    test_env::mock_app,
};
use portal::msg::{
    AssignVisaMsg, ExecuteMsg as PortalExecuteMsg, InstantiateMsg as PortalInitMsg,
    VisaAdminDetails,
};
use universe::species::{SapienceScale, Sapient};

#[test]
pub fn visa_is_approved() {
    /***** Setup environment *****/
    let mut app = mock_app();
    let jr_code = app.store_code(JumpRingContract::contract_code());
    let v_code = app.store_code(VisaContract::contract_code());

    let jr_init_msg = PortalInitMsg {
        planet_name: "foobar".to_string(),
        planet_sapients: vec![Sapient {}],
        minimum_sapience: SapienceScale::Medium,
    };

    let jr_ring_addr = app
        .instantiate_contract(
            jr_code,
            Addr::unchecked(JR_OWNER),
            &jr_init_msg,
            &[],
            JR_OWNER,
            None,
        )
        .unwrap();

    let jump_ring_contract = JumpRingContract(jr_ring_addr);

    let visa_init_msg = CW721InstantiateMsg {
        apes: vec![
            Addr::unchecked(APE_MINTER),
            Addr::unchecked(APE_CONTRACT_OWNER),
        ],
        name: VISA_HOLDER_NAME.to_string(),
        symbol: VISA_SYM.to_string(),
        jump_ring: jump_ring_contract.addr(),
    };

    let visa_contract_addr = app
        .instantiate_contract(
            v_code,
            Addr::unchecked(APE_CONTRACT_OWNER),
            &visa_init_msg,
            &[],
            APE_MINTER,
            None,
        )
        .unwrap();
    let visa_contract = VisaContract(visa_contract_addr);

    /***** Actual testing *****/
    // First, mint a token; this assumes the Ape found sufficient reason to allow for the visa.
    // This remains on the NFT contract; it will need to go through some steps before the JR will work.
    let token_id = TOKEN_ID_1.to_string();
    let token_uri = TOKEN_URI_1.to_string();

    let mint_msg: CW721ExecuteMsg = CW721ExecuteMsg::Mint(MintMsg::<Option<VisaMetadata>> {
        token_id: token_id.clone(),
        owner: String::from(APE_CONTRACT_OWNER),
        token_uri: Some(token_uri),
        extension: Some(VisaMetadata {
            account: Some(Addr::unchecked(VISA_HOLDER_ACCT)),
            name: Some(VISA_HOLDER_NAME.to_string()),
            species: Some(get_species_by_level(OTHER_ALIEN, SapienceScale::High)),
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

    // Second, the JR needs to whitelist the token_id so that it can later be checked against.
    // The Ape will be the entity to send the whitelist command via `AssignVisa` on the JR contract.
    let msg = PortalExecuteMsg::AssignVisa {
        msg: AssignVisaMsg {
            details: VisaAdminDetails {
                ape: Addr::unchecked(APE_ADMIN),
                contract: visa_contract.addr(),
                holder: Addr::unchecked(VISA_HOLDER_ACCT),
                token_id: token_id.clone(),
            },
        },
    };

    let res = app
        .execute_contract(
            Addr::unchecked(APE_ADMIN),
            jump_ring_contract.addr(),
            &msg,
            &[],
        )
        .unwrap();
    res.assert_event(&Event::new("wasm").add_attribute("action", "assign_visa"));

    // Third, the above is an unapproved contract, so the user cannot initiate a JumpRingTravel call.
    // Check to see that this attempt does not work.
    app.execute_contract(
        Addr::unchecked(APE_ADMIN),
        jump_ring_contract.addr(),
        &PortalExecuteMsg::JumpRingTravel {
            to: Addr::unchecked(DESTINATION),
        },
        &[],
    )
    .unwrap_err();

    // Fourth, the User (or Ape?) needs to send the NFT to the JR contract.
    let payload = Cw721ReceiveMsg {
        sender: VISA_HOLDER_ACCT.to_string(),
        token_id: token_id.clone(),
        msg: to_binary(BINARY_RCV_MSG).unwrap(),
    };

    let msg = PortalExecuteMsg::ReceiveNft(payload);
    let res = app
        .execute_contract(visa_contract.addr(), jump_ring_contract.addr(), &msg, &[])
        .unwrap();

    res.assert_event(&Event::new("wasm").add_attribute("action", "receive_visa"));
    res.assert_event(
        &Event::new("wasm").add_attribute("new_owner", jump_ring_contract.addr().to_string()),
    );
    res.assert_event(&Event::new("wasm").add_attribute("new_token_id", token_id.clone()));

    // Fifth, since this is a test for an approved visa, now the user should be able to travel.
    let res = app
        .execute_contract(
            Addr::unchecked(VISA_HOLDER_ACCT),
            jump_ring_contract.addr(),
            &PortalExecuteMsg::JumpRingTravel {
                to: Addr::unchecked(DESTINATION),
            },
            &[],
        )
        .unwrap();

    res.assert_event(&Event::new("wasm").add_attribute("action", "initiate_jump_ring_travel"))
}
