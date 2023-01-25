#[cfg(test)]
mod tests {
    use crate::{
        contract::{
            execute as hacker_execute, instantiate as hacker_instantiate, query as hacker_query,
            reply as hacker_reply,
        },
        msg::{
            ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg, SubscriberResponse,
        },
    };
    use cosmwasm_std::{to_binary, Addr, Empty, Uint128};
    use cw20::{BalanceResponse, Cw20Coin, Cw20ExecuteMsg, Cw20QueryMsg};
    use cw20_base::contract::{
        execute as cw20_execute, instantiate as cw20_instantiate, query as cw20_query,
    };
    use cw721::{Cw721QueryMsg, NftInfoResponse, NumTokensResponse};
    use cw721_metadata_onchain::{
        entry::{execute as cw721_execute, instantiate as cw721_instantiate, query as cw721_query},
        Metadata, Trait,
    };
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    // returns an object that can be used with cw-multi-test
    fn main_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(hacker_execute, hacker_instantiate, hacker_query)
            .with_reply(hacker_reply);
        Box::new(contract)
    }

    // returns an object that can be used with cw-multi-test
    fn cw721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(cw721_execute, cw721_instantiate, cw721_query);
        Box::new(contract)
    }

    // returns an object that can be used with cw-multi-test
    fn cw20_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(cw20_execute, cw20_instantiate, cw20_query);
        Box::new(contract)
    }

    const CONTRACT_OWNER: &str = "contract_owner";
    const SUBSCRIBER: &str = "subscriber";
    const HACKER: &str = "hacker";

    #[test]
    fn hack_process() {
        let contract_owner = Addr::unchecked(CONTRACT_OWNER);
        let subscriber = Addr::unchecked(SUBSCRIBER);
        let hacker = Addr::unchecked(HACKER);

        // define query balance function
        fn query_balance(app: &App, cw20_addr: Addr, addr: Addr) -> Uint128 {
            let query_msg = Cw20QueryMsg::Balance {
                address: addr.to_string(),
            };
            let balance: BalanceResponse = app
                .wrap()
                .query_wasm_smart(cw20_addr.clone(), &query_msg)
                .unwrap();
            balance.balance
        }

        // an app object is the blockchain simulator. we send initial balance here too!
        let mut app = App::new(|_router, _api, _storage| {});

        // upload the contracts to the blockchain and get back code_id to instantiate the contract
        let contract_code_id = app.store_code(main_contract());
        let cw721_code_id = app.store_code(cw721_contract());
        let cw20_code_id = app.store_code(cw20_contract());

        // instantiate main contract
        let instantiate_msg = InstantiateMsg {
            protocol_fee: 0,
            cw721_code_id: cw721_code_id,
            cw721_name: "White Hat Hacker NFT".to_string(),
            cw721_symbol: "WHH".to_string(),
            cw721_label: "White Hat Hacker Cw721".to_string(),
            cw721_admin: Some("cw721_contract_owner".to_string()),
        };
        let main_contract_addr = app
            .instantiate_contract(
                contract_code_id,
                contract_owner.clone(),
                &instantiate_msg,
                &[],
                "White Hat Hacker main contract",
                None,
            )
            .unwrap();

        // manually instantiate a cw20 contract and send the subscriber 1CWT
        let cw20_instantiate_msg = cw20_base::msg::InstantiateMsg {
            name: "CW20".to_string(),
            symbol: "CWT".to_string(),
            decimals: 6,
            initial_balances: vec![Cw20Coin {
                address: subscriber.to_string(),
                amount: Uint128::from(1_000_000u128),
            }],
            mint: None,
            marketing: None,
        };
        let cw20_addr = app
            .instantiate_contract(
                cw20_code_id,
                contract_owner.clone(),
                &cw20_instantiate_msg,
                &[],
                "Cw20".to_string(),
                None,
            )
            .unwrap();

        // query config
        let query_msg = QueryMsg::Config {};
        let config: ConfigResponse = app
            .wrap()
            .query_wasm_smart(main_contract_addr.clone(), &query_msg)
            .unwrap();

        assert_eq!(config.contract_owner, "contract_owner");
        assert_eq!(config.cw721_addr, "contract1");
        assert_eq!(config.protocol_fee, 0);

        // DAO subscribes to the contract
        let execute_msg = ExecuteMsg::Subscribe {
            subscriber: subscriber.clone(),
            bounty_pct: 20,
            min_bounty: None,
        };
        app.execute_contract(
            subscriber.clone(),
            main_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

        // query subscriber
        let query_msg = QueryMsg::Subscriber {
            protected_addr: subscriber.to_string(),
        };
        let res: SubscriberResponse = app
            .wrap()
            .query_wasm_smart(main_contract_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(res.bounty_pct, 20);
        assert_eq!(res.min_bounty, None);

        // query balance of the hacker before the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), hacker.clone()),
            Uint128::zero()
        );

        // query balance of the subscriber before the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), subscriber.clone()),
            Uint128::from(1_000_000u128)
        );

        // simulate hacker hacking the contract (subscriber sends 0.5 tokens to hacker)
        let execute_msg = Cw20ExecuteMsg::Transfer {
            recipient: hacker.to_string(),
            amount: Uint128::from(500_000u128),
        };
        app.execute_contract(subscriber.clone(), cw20_addr.clone(), &execute_msg, &[])
            .unwrap();

        // query balance of the hacker right after the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), hacker.clone()),
            Uint128::from(500_000u128)
        );

        // query balance of the subscriber right after the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), subscriber.clone()),
            Uint128::from(500_000u128)
        );

        // Hacker transfers the stolen tokens to the main contract
        let send_msg = to_binary(&ReceiveMsg::DepositCw20 {
            subscriber: subscriber.to_string(),
        })
        .unwrap();
        let msg = Cw20ExecuteMsg::Send {
            contract: main_contract_addr.to_string(),
            amount: Uint128::from(500_000u128),
            msg: send_msg.clone(),
        };
        app.execute_contract(hacker.clone(), cw20_addr.clone(), &msg, &[])
            .unwrap();

        // query balance of the hacker after giving hacked funds back to the contract
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), hacker.clone()),
            Uint128::from(100_000u128)
        );

        // query balance of the subscriber after giving hacked funds back to the contract
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), subscriber.clone()),
            Uint128::from(900_000u128)
        );

        // query number of NFTs minted after the hack (should be 1)
        let query_msg = Cw721QueryMsg::NumTokens {};
        let res: NumTokensResponse = app
            .wrap()
            .query_wasm_smart(config.cw721_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(res.count, 1);

        // let's hack with new protocol fee 10% (we need to update config first)
        let execute_msg = ExecuteMsg::UpdateConfig {
            new_contract_owner: Some("nahem".to_string()),
            new_bounty_pct: Some(10),
        };
        app.execute_contract(
            contract_owner.clone(),
            main_contract_addr.clone(),
            &execute_msg,
            &[],
        )
        .unwrap();

        // query config
        let query_msg = QueryMsg::Config {};
        let config: ConfigResponse = app
            .wrap()
            .query_wasm_smart(main_contract_addr.clone(), &query_msg)
            .unwrap();

        assert_eq!(config.contract_owner, "nahem");
        assert_eq!(config.cw721_addr, "contract1");
        assert_eq!(config.protocol_fee, 10);

        // simulate hacker hacking the contract (subscriber sends 0.9 tokens to hacker)
        let execute_msg = Cw20ExecuteMsg::Transfer {
            recipient: hacker.to_string(),
            amount: Uint128::from(900_000u128),
        };
        app.execute_contract(subscriber.clone(), cw20_addr.clone(), &execute_msg, &[])
            .unwrap();

        // query balance of the hacker right after the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), hacker.clone()),
            Uint128::from(100_000u128 + 900_000u128),
        );

        // query balance of the subscriber right after the hack
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), subscriber.clone()),
            Uint128::zero(),
        );

        // Hacker transfers the stolen tokens to the main contract
        let send_msg = to_binary(&ReceiveMsg::DepositCw20 {
            subscriber: subscriber.to_string(),
        })
        .unwrap();
        let msg = Cw20ExecuteMsg::Send {
            contract: main_contract_addr.to_string(),
            amount: Uint128::from(900_000u128),
            msg: send_msg.clone(),
        };
        app.execute_contract(hacker.clone(), cw20_addr.clone(), &msg, &[])
            .unwrap();

        // query balance of the hacker after giving hacked funds back to the contract
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), hacker.clone()),
            Uint128::from(100_000u128 + 180_000u128)
        );

        // query balance of the subscriber after giving hacked funds back to the contract
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), subscriber.clone()),
            Uint128::from(630_000u128)
        );

        // query balance of the main contract after paying everyone
        assert_eq!(
            query_balance(&app, cw20_addr.clone(), main_contract_addr.clone()),
            Uint128::from(90_000u128)
        );

        // query number of NFTs minted after the hack (should be 2)
        let query_msg = Cw721QueryMsg::NumTokens {};
        let res: NumTokensResponse = app
            .wrap()
            .query_wasm_smart(config.cw721_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(res.count, 2);

        // query NFT onchain metadata with custom attributes & Traits
        let query_msg = Cw721QueryMsg::NftInfo {
            token_id: "2".to_string(),
        };
        let res: NftInfoResponse<Metadata> = app
            .wrap()
            .query_wasm_smart(config.cw721_addr.clone(), &query_msg)
            .unwrap();
        assert_eq!(
            res.extension.attributes,
            Some(vec![
                Trait {
                    display_type: None,
                    trait_type: "date".to_string(),
                    value: "1571797419".to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "total_amount_hacked".to_string(),
                    value: 900_000u128.to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "bounty".to_string(),
                    value: 180_000u128.to_string(),
                },
                Trait {
                    display_type: None,
                    trait_type: "hacker_addr".to_string(),
                    value: HACKER.to_string(),
                },
            ])
        );
    }
}
