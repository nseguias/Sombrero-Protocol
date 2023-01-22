use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_fee: u16,
    pub min_bounty: Option<u128>,
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
        protected_addr: Addr,
        bounty_pct: u16,
        min_bounty: Option<u128>,
    },
    Receive {
        cw20_msg: Cw20ReceiveMsg,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SubscriberResponse)]
    Subscriber { protected_addr: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub contract_owner: Addr,
    pub protocol_fee: u16,
    pub cw721_contract_addr: Addr,
}

#[cw_serde]
pub struct SubscriberResponse {
    pub bounty_pct: u16,
    pub min_bounty: Option<u128>,
}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub enum ReceiveMsg {
    DepositCw20 {},
}
