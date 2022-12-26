use cosmwasm_schema::write_api;
use cw721_visa::{
    contract::{ExecuteMsg, QueryMsg},
    msg::InstantiateMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
