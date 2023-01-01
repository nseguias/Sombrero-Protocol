use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub contract_owner: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub enum Action {
    None,
    Freeze,
}

#[cw_serde]
pub struct Conditions {
    pub subscriber: Addr,
    pub commission: String,
    pub action: Action,
}

pub const CONDITIONS: Map<Addr, Conditions> = Map::new("conditions");
