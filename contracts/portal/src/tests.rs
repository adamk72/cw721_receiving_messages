#![cfg(test)]

use cosmwasm_std::testing::mock_dependencies;
use cosmwasm_std::{testing::mock_info, Addr, Response};

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
