use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage, SubMsg, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::{
    msg::Cw20HookMsg,
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

pub fn subscribe(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    commission_bps: u16,
) -> Result<Response, ContractError> {
    // Protocol owner, DAO or auiting firm can subscribe to the contract
    if commission_bps > 10000 {
        return Err(ContractError::InvalidCommissionBps {});
    }
    let subscriptions = Subscriptions { commission_bps };
    SUBSCRIPTIONS.save(deps.storage, info.sender, &subscriptions)?;

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
) -> StdResult<Response> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::DepositCw20 {}) => deposit_cw20(deps, env, info, cw20_msg),
        _ => Err(StdError::generic_err("error parsing instantiate reply")),
    }
}

pub fn deposit_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let hacker_addr = deps.api.addr_validate(&receive_msg.sender)?;
    let cw20_contract = info.sender;

    // TODO: check if enough native tokens are sent to avoid spamming

    let subscriptions = SUBSCRIPTIONS.load(deps.storage, cw20_contract)?;

    let bounty = subscriptions.commission_bps as u128 * receive_msg.amount.u128() / 10000;

    let config = CONFIG.load(deps.storage)?;

    let msgs = mint_nft(
        deps.storage,
        config.cw721_code_id,
        &env.contract.address,
        &env.contract.address,
        bounty,
        hacker_addr,
    )?;
    //
    // let mint_msg = MintMsg { token_id, owner, token_uri, extension };

    Ok(Response::new()
        .add_attribute("action", "deposit_cw20")
        // Q: should I use submessage or message?
        .add_submessages(msgs))
}

pub fn mint_nft(
    _storage: &mut dyn Storage,
    cw721_code_id: u64,
    cw721_contract_addr: &Addr,
    _minter: &Addr,
    bounty: u128,
    hacker_addr: Addr,
) -> StdResult<Vec<SubMsg>> {
    let mut msgs: Vec<SubMsg> = vec![];

    // Q: This should be something like Cw721ExecuteMsg::Mint...
    let cw721_mint_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw721_contract_addr.to_string(),
        // Q: what would be the CW721 equivalent of Mint?
        msg: to_binary(&Cw20ExecuteMsg::Mint {
            recipient: hacker_addr.to_string(),
            amount: bounty.into(),
        })?,
        funds: vec![],
    });

    // This T errors out
    // let mint_msg: MintMsg<T> = MintMsg {
    //     token_id: "1".to_string(),
    //     owner: hacker_addr.to_string(),
    //     token_uri: Some("https://example.com".to_string()),
    //     extension: None,
    // };

    msgs.push(SubMsg::reply_on_success(cw721_mint_msg, cw721_code_id));

    Ok(msgs)
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
        // Q: not sure if this should / can be updated
        cw721_code_id: config.cw721_code_id,
        cw721_contract_addr: config.cw721_contract_addr,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
