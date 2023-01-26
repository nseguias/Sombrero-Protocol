use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub contract_owner: Addr,
    pub protocol_fee: u16,
    pub cw721_addr: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Subscriptions {
    pub subscriber: Addr,
    pub bounty_pct: u16,
    pub min_bounty: Option<u128>,
}

#[cw_serde]
pub struct Hacks {
    pub date: u64,
    pub contract_exploited: Addr,
    pub total_amount_hacked: Uint128,
    pub bounty: Uint128,
    pub hacker_addr: Addr,
}

pub const SUBSCRIPTIONS: Map<Addr, Subscriptions> = Map::new("conditions");
pub const HACKS: Map<(Addr, u64), Hacks> = Map::new("hacks");
