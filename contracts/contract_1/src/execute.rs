use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{
    state::{Config, Subscriptions, CONFIG, SUBSCRIPTIONS},
    ContractError,
};

pub fn boilerplate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "boilerplate"))
}

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_contract_owner: Option<String>,
    protocol_fee_bps: Option<u16>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    if new_contract_owner.is_none() && protocol_fee_bps.is_none() {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_contract_owner == Some(config.contract_owner.to_string())
        && protocol_fee_bps == Some(config.protocol_fee_bps)
    {
        return Err(ContractError::NothingToUpdate {});
    }

    let val_new_contract_owner = deps
        .api
        .addr_validate(&new_contract_owner.unwrap_or(config.contract_owner.to_string()));

    let config = Config {
        contract_owner: val_new_contract_owner?,
        protocol_fee_bps: protocol_fee_bps.unwrap_or(config.protocol_fee_bps),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn subscribe(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    protected_contract: String,
    beneficiary: String,
    commission_bps: u16,
) -> Result<Response, ContractError> {
    if beneficiary == protected_contract {
        return Err(ContractError::BeneficiaryMustBeDifferentFromProtectedContract {});
    }
    if info.sender != protected_contract {
        return Err(ContractError::Unauthorized {});
    }
    if commission_bps > 10000 {
        return Err(ContractError::InvalidCommissionBps {});
    }
    let subscriptions = Subscriptions {
        beneficiary: deps.api.addr_validate(&beneficiary)?,
        commission_bps,
    };
    SUBSCRIPTIONS.save(
        deps.storage,
        deps.api.addr_validate(&protected_contract)?,
        &subscriptions,
    )?;

    Ok(Response::new().add_attribute("action", "subscribe"))
}
