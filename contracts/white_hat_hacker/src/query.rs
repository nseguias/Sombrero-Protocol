use cosmwasm_std::{Deps, StdResult};

use crate::msg::{ConfigResponse, SubscriberResponse};
use crate::state::{CONFIG, SUBSCRIPTIONS};

pub fn subscriber(deps: Deps, protected_addr: String) -> StdResult<SubscriberResponse> {
    let subscriptions =
        SUBSCRIPTIONS.load(deps.storage, deps.api.addr_validate(&protected_addr)?)?;

    Ok(SubscriberResponse {
        bounty_pct: subscriptions.bounty_pct,
        min_bounty: subscriptions.min_bounty,
    })
}

pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        contract_owner: cfg.contract_owner,
        protocol_fee: cfg.protocol_fee,
        cw721_addr: cfg.cw721_addr,
    })
}
