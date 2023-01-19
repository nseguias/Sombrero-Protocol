#[cfg(test)]
mod tests {
    use crate::{
        contract::{
            execute as hacker_execute, instantiate as hacker_instantiate, query as hacker_query,
            reply as hacker_reply,
        },
        msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, SubscriberResponse},
    };
    use cosmwasm_std::{coins, to_binary, Addr, Empty, Uint128};
    use cw20::Cw20ExecuteMsg;
    use cw721_base::entry::execute as cw721_execute;
    use cw721_base::entry::instantiate as cw721_instantiate;
    use cw721_base::entry::query as cw721_query;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    // returns an object that can be used with cw-multi-test
    fn hacker_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(hacker_execute, hacker_instantiate, hacker_query)
            .with_reply(hacker_reply);
        Box::new(contract)
    }

    // returns an object that can be used with cw-multi-test
    fn cw721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(cw721_execute, cw721_instantiate, cw721_query);
        Box::new(contract)
    }

    const DENOM: &str = "uATOM";

    #[test]
    fn hack_process() {
        let contract_owner = Addr::unchecked("contract_owner");
        let protected_addr = Addr::unchecked("protected_addr");
        let suscriber = Addr::unchecked("suscriber");
        let hacker = Addr::unchecked("hacker");
        let cw20_addr = Addr::unchecked("cw20_addr");

        // an app object is the blockchain simulator. we send initial balance here too!
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &contract_owner, coins(1_000_000u128, DENOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &protected_addr, coins(1_000_000u128, DENOM))
                .unwrap();
            router
                .bank
                .init_balance(storage, &hacker, coins(1_000_000u128, DENOM))
                .unwrap();
        });

        // upload the contract to the blockchain and get back code_id to instantiate the contract
        let contract_code_id = app.store_code(hacker_contract());
        let cw721_code_id = app.store_code(cw721_contract());

        // instantiate cw20 contract
        let instantiate_msg = InstantiateMsg {
            protocol_fee: 0,
            min_bounty: None,
            cw721_code_id: cw721_code_id,
            cw721_name: "White Hat Hacker NFT".to_string(),
            cw721_symbol: "WHH".to_string(),
            cw721_label: "White Hat Hacker Cw721".to_string(),
            cw721_admin: Some("cw721_contract_owner".to_string()),
        };

        let hacker_contract_addr = app
            .instantiate_contract(
                contract_code_id,
                contract_owner.clone(),
                &instantiate_msg,
                &coins(1_000_000u128, DENOM),
                "White Hat Hacker Cw20",
                None,
            )
            .unwrap();

        // DAO subscribes to the contract
        let execute_msg = ExecuteMsg::Subscribe {
            protected_addr: protected_addr.clone(),
            bounty_pct: 20,
            min_bounty: None,
        };
        app.execute_contract(
            suscriber.clone(),
            hacker_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

        // query balance of the hacker before hacking the contract
        let hacker_balance = app.wrap().query_balance(hacker.clone(), DENOM).unwrap();
        assert_eq!(hacker_balance.amount.u128(), 1_000_000u128);

        // simulate hack: hacker gets funds from protected contract
        app.send_tokens(
            protected_addr.clone(),
            hacker.clone(),
            &coins(500_000u128, DENOM),
        )
        .unwrap();

        // TODO: I might need to create a Cw20ExecuteMsg::Send{} to send tokens to the contract with a message
        // Hacker transfers the stolen tokens to the contract
        let execute_msg = Cw20ExecuteMsg::Send {
            contract: cw20_addr.to_string(),
            amount: Uint128::from(500_000u128),
            msg: to_binary(&0).unwrap(),
        };
        app.execute_contract(
            hacker.clone(),
            hacker_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

        // // hacker sends stolen tokens to the contract -> NOT WORKING
        // app.send_tokens(
        //     hacker.clone(),
        //     hacker_contract_addr.clone(),
        //     &coins(500_000u128, DENOM),
        // )
        // .unwrap();

        // TODO: Contract is not sending tokens back to hacker
        // query balance of the hacker after hacking the contract
        let hacker_balance = app.wrap().query_balance(hacker.clone(), DENOM).unwrap();

        assert_eq!(hacker_balance.amount.u128(), 1_100_000u128);

        // query balance of the DAO after being hacked
        let subscriber_balance = app.wrap().query_balance(suscriber.clone(), DENOM).unwrap();

        assert_eq!(subscriber_balance.amount.u128(), 900_000u128);

        // query config
        let query_msg = QueryMsg::Config {};
        let config_res: ConfigResponse = app
            .wrap()
            .query_wasm_smart(hacker_contract_addr.clone(), &query_msg)
            .unwrap();

        assert_eq!(config_res.contract_owner, "contract_owner");
        assert_eq!(config_res.cw721_contract_addr, "contract1");
        assert_eq!(config_res.protocol_fee, 0);

        let query_msg = QueryMsg::Subscriber {
            protected_addr: protected_addr.to_string(),
        };
        let res: SubscriberResponse = app
            .wrap()
            .query_wasm_smart(hacker_contract_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(res.bounty_pct, 20);
        assert_eq!(res.min_bounty, None);
    }
}
