use cosmwasm_std::{Deps, StdResult};

use crate::{msg::BoilerplateResponse, state::CONFIG};

pub fn boilerplate(deps: Deps) -> StdResult<BoilerplateResponse> {
    let _cfg = CONFIG.load(deps.storage)?;

    Ok(BoilerplateResponse {})
}
