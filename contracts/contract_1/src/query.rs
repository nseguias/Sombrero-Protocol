use cosmwasm_std::{Deps, StdResult};

use crate::msg::{BoilerplateResponse, SubscriberResponse};
use crate::state::{CONFIG, SUBSCRIPTIONS};

pub fn boilerplate(deps: Deps) -> StdResult<BoilerplateResponse> {
    let _cfg = CONFIG.load(deps.storage)?;

    Ok(BoilerplateResponse {})
}

pub fn subscriber(deps: Deps, protected_addr: String) -> StdResult<SubscriberResponse> {
    let subscriptions =
        SUBSCRIPTIONS.load(deps.storage, deps.api.addr_validate(&protected_addr)?)?;

    Ok(SubscriberResponse {
        bounty_pct: subscriptions.bounty_pct,
        min_bounty: subscriptions.min_bounty,
    })
}
