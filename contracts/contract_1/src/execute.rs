use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{state::CONFIG, ContractError};

pub fn boilerplate(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    if info.sender == cfg.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new().add_attribute("action", "execute_boilerplate"))
}

pub fn subscribe(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;

    if info.sender == cfg.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new().add_attribute("action", "execute_boilerplate"))
}
