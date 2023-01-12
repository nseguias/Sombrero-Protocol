#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        msg::{BoilerplateResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
    };

    // returns an object that can be used with cw-multi-test
    fn boilerplate_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    const DENOM: &str = "denom";
    pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

    #[test]
    fn boilerplate_process() {
        let owner = Addr::unchecked("owner");
        let user = Addr::unchecked("user");

        // an app object is the blockchain simulator. we send initial balance here too!
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &owner, coins(2_000_000u128, DENOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &user, coins(20_000_000u128, DENOM))
                .unwrap();
        });

        // upload the contract to the blockchain and get back code_id to instantiate the contract
        let code_id = app.store_code(boilerplate_contract());

        // instantiate
        let instantiate_msg = InstantiateMsg {
            protocol_fee_bps: 0,
            cw721_code_id: INSTANTIATE_CW721_REPLY_ID,
            cw721_name: "NAME".to_string(),
            cw721_symbol: "SYMBOL".to_string(),
            cw721_minter: "minter".to_string(),
        };
        let contract_addr = app
            .instantiate_contract(
                code_id,
                owner.clone(),
                &instantiate_msg,
                &coins(2_000_000u128, DENOM),
                "Boilerplate",
                None,
            )
            .unwrap();

        // execute
        let execute_msg = ExecuteMsg::Boilerplate {};
        app.execute_contract(
            user.clone(),
            contract_addr.clone(),
            &execute_msg,
            &coins(15_000_000u128, DENOM),
        )
        .unwrap();

        // query
        let query_msg = QueryMsg::Boilerplate {};
        let _res: BoilerplateResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_msg)
            .unwrap();
    }
}
