use cosmwasm_schema::{cw_serde, QueryResponses};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_fee_bps: u16,
    pub cw721_code_id: u64,
    pub cw721_name: String,
    pub cw721_symbol: String,
    pub cw721_minter: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Boilerplate {},
    UpdateConfig {
        new_contract_owner: Option<String>,
        new_protocol_fee_bps: Option<u16>,
    },
    Subscribe {
        commission_bps: u16,
        // A basis point (bps) is one one-hundredth of a percent (0.01%). For example, 100 basis points equal 1%
    },
    Receive {
        cw20_msg: Cw20ReceiveMsg,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(BoilerplateResponse)]
    Boilerplate {},
}

#[cw_serde]
pub struct BoilerplateResponse {}

#[cw_serde]
pub enum MigrateMsg {}

#[cw_serde]
pub enum Cw20HookMsg {
    DepositCw20 {},
}
