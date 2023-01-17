use cosmwasm_std::Empty;
use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw721_base::Extension;
use cw721_base::MintMsg;

use crate::{
    msg::Cw20HookMsg,
    state::{Config, Subscriptions, CONFIG, SUBSCRIPTIONS},
    ContractError,
};

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

pub fn boilerplate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "boilerplate"))
}

pub fn subscribe(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    commission_bps: u16,
    subscribe_contract: Addr,
) -> Result<Response, ContractError> {
    // Protocol owner, DAO or auiting firm can subscribe to the contract
    if commission_bps > 10000 {
        return Err(ContractError::InvalidCommissionBps {});
    }
    let subscriptions = Subscriptions { commission_bps };
    SUBSCRIPTIONS.save(deps.storage, subscribe_contract, &subscriptions)?;

    Ok(Response::new().add_attribute("action", "subscribe"))
}

pub fn unsubscribe(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    SUBSCRIPTIONS.remove(deps.storage, info.sender);

    Ok(Response::new().add_attribute("action", "unsubscribe"))
}

pub fn handle_receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // validate that cw20 contract is sending this message
    let config = CONFIG.load(deps.storage)?;
    if config.contract_owner != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::DepositCw20 {}) => deposit_cw20(deps, env, info, cw20_msg),
        _ => Err(ContractError::ErrorParsingInstantiateReply {}),
    }
}

pub fn deposit_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let hacker_addr = deps.api.addr_validate(&receive_msg.sender)?;
    let cw20_contract = info.sender.clone();
    let subscriptions = SUBSCRIPTIONS.load(deps.storage, cw20_contract.clone())?;
    let bounty = subscriptions.commission_bps as u128 * receive_msg.amount.u128() / 10000;

    let mut messages = Vec::new();

    // transfer bounty to hacker as a message
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_contract.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: hacker_addr.to_string(),
            amount: bounty.into(),
        })?,
        funds: vec![],
    }));

    // Whose address is this?
    let whose_address = env.contract.address.clone();
    // transfer remaining funds to suscriber as a message
    // TODO: check recipient address! &
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_contract.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: whose_address.to_string(),
            amount: (receive_msg.amount.u128() - bounty).into(),
        })?,
        funds: vec![],
    }));

    let config = CONFIG.load(deps.storage)?;
    let num_tokens: u64 = deps
        .querier
        .query_wasm_smart(config.cw721_contract_addr, &QueryMsg::NumTokens {})?;

    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_contract.to_string(),
        msg: to_binary(&ExecuteMsg::Mint(MintMsg::<Extension> {
            token_id: (num_tokens + 1).to_string(),
            owner: hacker_addr.to_string(),
            token_uri: None,
            extension: None,
        }))?,
        funds: vec![],
    }));

    Ok(Response::new()
        .add_attribute("action", "deposit_cw20")
        .add_messages(messages))
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

pub fn update_subscription(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_commission_bps: Option<u16>,
) -> Result<Response, ContractError> {
    let subscriptions = SUBSCRIPTIONS.load(deps.storage, info.sender.clone())?;

    if new_commission_bps.is_none() {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_commission_bps > Some(10000) {
        return Err(ContractError::InvalidCommissionBps {});
    }

    let subscriptions = Subscriptions {
        commission_bps: new_commission_bps.unwrap_or(subscriptions.commission_bps),
    };
    SUBSCRIPTIONS.save(deps.storage, info.sender, &subscriptions)?;

    Ok(Response::new().add_attribute("action", "update_subscription"))
}

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_contract_owner: Option<String>,
    new_bounty_pct: Option<u16>,
    new_min_bounty: Option<u128>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    if new_contract_owner.is_none()
        && new_bounty_pct.is_none()
        && new_min_bounty == config.min_bounty
    {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_contract_owner == Some(config.contract_owner.to_string())
        && new_bounty_pct == Some(config.bounty_pct)
        && new_min_bounty == config.min_bounty
    {
        return Err(ContractError::NothingToUpdate {});
    }

    let val_new_contract_owner = deps
        .api
        .addr_validate(&new_contract_owner.unwrap_or(config.contract_owner.to_string()));

    let config = Config {
        contract_owner: val_new_contract_owner?,
        bounty_pct: new_bounty_pct.unwrap_or(config.bounty_pct),
        min_bounty: new_min_bounty,
        cw721_contract_addr: config.cw721_contract_addr,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
