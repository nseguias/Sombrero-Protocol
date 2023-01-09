use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub contract_owner: Addr,
    pub cw721_code_id: u64,
    pub protocol_fee_bps: u16,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Subscriptions {
    pub beneficiary: Addr,
    pub commission_bps: u16,
}

#[cw_serde]
pub struct Hacker {
    pub balance: u128,
    pub recipient: Addr,
    pub give_up_bounty: bool,
    pub counter_offer: u128,
}

pub const SUBSCRIPTIONS: Map<Addr, Subscriptions> = Map::new("conditions");
pub const BALANCES: Map<(Addr, Addr), Hacker> = Map::new("balances");
