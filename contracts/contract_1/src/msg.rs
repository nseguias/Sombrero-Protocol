use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub protocol_fee_bps: u16,
}

#[cw_serde]
pub enum ExecuteMsg {
    Boilerplate {},
    UpdateConfig {
        new_contract_owner: Option<String>,
        new_protocol_fee_bps: Option<u16>,
    },
    Subscribe {
        beneficiary: String,
        commission_bps: u16,
        // A basis point (bps) is one one-hundredth of a percent (0.01%). For example, 100 basis points equal 1%
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
