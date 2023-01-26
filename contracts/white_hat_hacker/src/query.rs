use cosmwasm_std::{Deps, Order, StdResult};

use crate::msg::{ConfigResponse, SubscriberResponse, SubscriptionsResponse};
use crate::state::{CONFIG, SUBSCRIPTIONS};

pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        contract_owner: cfg.contract_owner,
        protocol_fee: cfg.protocol_fee,
        cw721_addr: cfg.cw721_addr,
    })
}

pub fn subscriber(deps: Deps, protected_addr: String) -> StdResult<SubscriberResponse> {
    let subscriptions =
        SUBSCRIPTIONS.load(deps.storage, deps.api.addr_validate(&protected_addr)?)?;

    Ok(SubscriberResponse {
        bounty_pct: subscriptions.bounty_pct,
        min_bounty: subscriptions.min_bounty,
    })
}

pub fn subscriptions(deps: Deps) -> StdResult<Vec<SubscriptionsResponse>> {
    let subscriptions = SUBSCRIPTIONS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (addr, subscriptions) = item?;
            Ok(SubscriptionsResponse {
                subscriber: addr,
                bounty_pct: subscriptions.bounty_pct,
                min_bounty: subscriptions.min_bounty,
            })
        })
        .collect::<StdResult<Vec<SubscriptionsResponse>>>()?;

    Ok(subscriptions)
}
