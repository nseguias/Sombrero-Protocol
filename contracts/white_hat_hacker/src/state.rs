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
pub struct Hacker {
    pub balance: u128,
    pub amount_hacked: u128,
    pub bounty_received: u128,
}

#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub date_time: Option<String>,
    pub hacked_amount: Option<Uint128>,
    pub bounty_received: Option<Uint128>,
    pub hacker: Option<String>,
}

pub const SUBSCRIPTIONS: Map<Addr, Subscriptions> = Map::new("conditions");
pub const BALANCES: Map<(Addr, Addr), Hacker> = Map::new("balances");
