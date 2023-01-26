use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_fee: u16,
    pub cw721_code_id: u64,
    pub cw721_name: String,
    pub cw721_symbol: String,
    pub cw721_label: String,
    pub cw721_admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        new_contract_owner: Option<String>,
        new_bounty_pct: Option<u16>,
    },
    Subscribe {
        subscriber: Addr,
        bounty_pct: u16,
        min_bounty: Option<u128>,
    },
    UpdateSubscription {
        subscriber: String,
        new_bounty_pct: Option<u16>,
        new_min_bounty: Option<u128>,
    },
    Unsubscribe {
        subscriber: String,
    },
    Receive(Cw20ReceiveMsg),
    Withdraw {
        cw20_addr: String,
        amount: u128,
        recipient: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SubscriptionResponse)]
    Subscription { protected_addr: String },
    #[returns(SubscriptionsResponse)]
    Subscriptions {},
    #[returns(HacksResponse)]
    Hacks {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub contract_owner: Addr,
    pub protocol_fee: u16,
    pub cw721_addr: Addr,
}

#[cw_serde]
pub struct SubscriptionResponse {
    pub bounty_pct: u16,
    pub min_bounty: Option<u128>,
}

#[cw_serde]
pub struct SubscriptionsResponse {
    pub subscriber: Addr,
    pub bounty_pct: u16,
    pub min_bounty: Option<u128>,
}

#[cw_serde]
pub struct HacksResponse {
    pub date: u64,
    pub contract_exploited: Addr,
    pub total_amount_hacked: Uint128,
    pub bounty: Uint128,
    pub hacker_addr: Addr,
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub enum ReceiveMsg {
    DepositCw20 { subscriber: String },
}
