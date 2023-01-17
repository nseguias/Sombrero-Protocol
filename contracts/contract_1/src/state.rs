use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub contract_owner: Addr,
    pub protocol_fee: u16,
    pub cw721_contract_addr: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct Subscriptions {
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
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub const SUBSCRIPTIONS: Map<Addr, Subscriptions> = Map::new("conditions");
pub const BALANCES: Map<(Addr, Addr), Hacker> = Map::new("balances");
