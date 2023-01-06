use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw20::Cw20ReceiveMsg;

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
    new_protocol_fee_bps: Option<u16>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    if new_contract_owner.is_none() && new_protocol_fee_bps.is_none() {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_contract_owner == Some(config.contract_owner.to_string())
        && new_protocol_fee_bps == Some(config.protocol_fee_bps)
    {
        return Err(ContractError::NothingToUpdate {});
    }

    let val_new_contract_owner = deps
        .api
        .addr_validate(&new_contract_owner.unwrap_or(config.contract_owner.to_string()));

    let config = Config {
        contract_owner: val_new_contract_owner?,
        protocol_fee_bps: new_protocol_fee_bps.unwrap_or(config.protocol_fee_bps),
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn subscribe(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    beneficiary: String,
    commission_bps: u16,
    balance: u128,
) -> Result<Response, ContractError> {
    if beneficiary == info.sender.to_string() {
        return Err(ContractError::BeneficiaryMustBeDifferentFromProtectedContract {});
    }
    if commission_bps > 10000 {
        return Err(ContractError::InvalidCommissionBps {});
    }
    let subscriptions = Subscriptions {
        beneficiary: deps.api.addr_validate(&beneficiary)?,
        commission_bps,
        balance,
    };
    SUBSCRIPTIONS.save(deps.storage, info.sender, &subscriptions)?;

    Ok(Response::new().add_attribute("action", "subscribe"))
}

pub fn update_subscription(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_beneficiary: Option<String>,
    new_commission_bps: Option<u16>,
    balance: u128,
) -> Result<Response, ContractError> {
    let subscriptions = SUBSCRIPTIONS.load(deps.storage, info.sender.clone())?;

    if new_beneficiary == Some(info.sender.to_string()) {
        return Err(ContractError::BeneficiaryMustBeDifferentFromProtectedContract {});
    }
    if new_beneficiary.is_none() && new_commission_bps.is_none()
        || (new_beneficiary == Some(subscriptions.beneficiary.to_string())
            && new_commission_bps == Some(subscriptions.commission_bps))
    {
        return Err(ContractError::NothingToUpdate {});
    }

    let val_new_beneficiary = deps
        .api
        .addr_validate(&new_beneficiary.unwrap_or(subscriptions.beneficiary.to_string()));

    if new_commission_bps > Some(10000) {
        return Err(ContractError::InvalidCommissionBps {});
    }

    let subscriptions = Subscriptions {
        beneficiary: val_new_beneficiary?,
        commission_bps: new_commission_bps.unwrap_or(subscriptions.commission_bps),
        balance: subscriptions.balance,
    };
    SUBSCRIPTIONS.save(deps.storage, info.sender, &subscriptions)?;

    Ok(Response::new().add_attribute("action", "update_subscription"))
}

pub fn unsubscribe(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    SUBSCRIPTIONS.remove(deps.storage, info.sender);

    Ok(Response::new().add_attribute("action", "unsubscribe"))
}

pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Option<u128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    let amount = amount.unwrap_or(0u128);

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount.to_string()))
}

/*
pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let sender = addr_validate_to_lower(deps.api, &cw20_msg.sender)?;
    match from_binary(&cw20_msg.msg)? {
        Cw20HookMsg::ExecuteSwapOperations {
            operations,
            minimum_receive,
            to,
            max_spread,
        } => execute_swap_operations(
            deps,
            env,
            sender,
            operations,
            minimum_receive,
            to,
            max_spread,
        ),
    }
}
*/
