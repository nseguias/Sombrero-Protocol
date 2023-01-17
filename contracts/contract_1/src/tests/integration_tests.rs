#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::{
        contract::{execute, instantiate, query},
        msg::{BoilerplateResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
    };

    // returns an object that can be used with cw-multi-test
    fn hacker_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    const DENOM: &str = "uATOM";
    pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

    #[test]
    fn boilerplate_process() {
        let cw20_contract_owner = Addr::unchecked("cw20_contract_owner");
        let suscriber = Addr::unchecked("suscriber");
        let hacker = Addr::unchecked("hacker");

        // an app object is the blockchain simulator. we send initial balance here too!
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &cw20_contract_owner, coins(1_000_000u128, DENOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &suscriber, coins(1_000_000u128, DENOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &hacker, coins(1_000_000u128, DENOM))
                .unwrap();
        });

        // upload the contract to the blockchain and get back code_id to instantiate the contract
        let cw20_code_id = app.store_code(hacker_contract());

        // instantiate cw20 contract
        let instantiate_msg = InstantiateMsg {
            bounty_pct: 0,
            min_bounty: None,
            cw721_code_id: INSTANTIATE_CW721_REPLY_ID,
            cw721_name: "NAME".to_string(),
            cw721_symbol: "SYMBOL".to_string(),
            cw721_label: "label".to_string(),
            cw721_admin: Some("contract_address".to_string()),
        };
        let contract_addr = app
            .instantiate_contract(
                cw20_code_id,
                cw20_contract_owner.clone(),
                &instantiate_msg,
                &coins(2_000_000u128, DENOM),
                "Boilerplate",
                None,
            )
            .unwrap();

        // execute
        let execute_msg = ExecuteMsg::Boilerplate {};
        app.execute_contract(
            suscriber.clone(),
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
