use crate::contract_helpers::ContractBase;
use cosmwasm_std::Addr;
use cw_multi_test::ContractWrapper;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VisaContract(pub Addr);

impl ContractBase for VisaContract {
    type ExecuteMsg = cw721_visa::contract::ExecuteMsg;

    fn addr(&self) -> Addr {
        self.0.clone()
    }

    fn contract_code() -> Box<dyn cw_multi_test::Contract<cosmwasm_std::Empty>> {
        let contract = ContractWrapper::new(
            cw721_visa::contract::execute,
            cw721_visa::contract::instantiate,
            cw721_visa::contract::query,
        );
        // .with_reply(cw721_visa::contract::reply);

        Box::new(contract)
    }
}

pub struct JumpRingContract(pub Addr);

impl ContractBase for JumpRingContract {
    type ExecuteMsg = portal::msg::ExecuteMsg;

    fn addr(&self) -> Addr {
        self.0.clone()
    }

    fn contract_code() -> Box<dyn cw_multi_test::Contract<cosmwasm_std::Empty>> {
        let contract = ContractWrapper::new(
            portal::contract::execute,
            portal::contract::instantiate,
            portal::contract::query,
        );
        // .with_reply(cw721_visa::contract::reply);

        Box::new(contract)
    }
}
