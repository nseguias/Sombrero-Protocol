use cosmwasm_std::Uint128;
use cosmwasm_std::{
    from_binary, to_binary, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg};
use cw721::NumTokensResponse;
use cw721_metadata_onchain::{Extension, Metadata, MintMsg, Trait};

use crate::msg::ReceiveMsg;
use crate::{
    state::{Config, Subscriptions, CONFIG, SUBSCRIPTIONS},
    ContractError,
};

// NOTE: this was ExecuteMsg & QueryMsg before, might have to change it back
pub type Cw721ExecuteMsg = cw721_metadata_onchain::ExecuteMsg;
pub type Cw721QueryMsg = cw721_metadata_onchain::QueryMsg;

pub fn subscribe(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    subscriber: Addr,
    bounty_pct: u16,
    min_bounty: Option<u128>,
) -> Result<Response, ContractError> {
    if bounty_pct > 100 {
        return Err(ContractError::InvalidBountyPercentage {});
    }
    // save subscription details on state
    let subscriptions = Subscriptions {
        subscriber: subscriber.clone(),
        bounty_pct,
        min_bounty,
    };
    SUBSCRIPTIONS.save(deps.storage, subscriber.clone(), &subscriptions)?;

    Ok(Response::new()
        .add_attribute("action", "subscribe")
        .add_attribute("subscriber", subscriber)
        .add_attribute("bounty_pct", bounty_pct.to_string())
        .add_attribute("min_bounty", min_bounty.unwrap_or(0u128).to_string()))
}

pub fn unsubscribe(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // sender will be removed from subscriptions (if exists)
    SUBSCRIPTIONS.remove(deps.storage, info.sender.clone());
    Ok(Response::new()
        .add_attribute("action", "unsubscribe")
        .add_attribute("unsubscribed", info.sender))
}

pub fn handle_receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // validate that cw20 contract is sending this message
    let hacker_addr = deps.api.addr_validate(&cw20_msg.sender)?;
    let cw20_addr = info.sender.clone();
    let msg: ReceiveMsg = from_binary(&cw20_msg.msg)?;

    match msg {
        ReceiveMsg::DepositCw20 { subscriber } => deposit_cw20(
            deps,
            env,
            subscriber,
            hacker_addr,
            cw20_addr,
            cw20_msg.amount,
        ),
    }
}

pub fn deposit_cw20(
    deps: DepsMut,
    env: Env,
    subscriber: String,
    hacker_addr: Addr,
    cw20_addr: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let subscriber = deps.api.addr_validate(&subscriber)?;
    let subscriptions = SUBSCRIPTIONS.load(deps.storage, subscriber.clone())?;
    let bounty = subscriptions.bounty_pct as u128 * amount.u128() / 100;
    let cfg = CONFIG.load(deps.storage)?;
    let mut messages = Vec::new();
    let config = CONFIG.load(deps.storage)?;
    let cw721_addr = config.cw721_addr;

    // transfer bounty to hacker as a message
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: hacker_addr.to_string(),
            amount: bounty.into(),
        })?,
        funds: vec![],
    }));

    // transfer remaining funds to suscriber as a message
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw20_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: subscriptions.subscriber.to_string(),
            amount: (amount.u128() - bounty - amount.u128() * cfg.protocol_fee as u128 / 100)
                .into(),
        })?,
        funds: vec![],
    }));

    // create NFT metadata with hack details
    let traits: Vec<Trait> = vec![
        Trait {
            display_type: None,
            trait_type: "date".to_string(),
            value: env.block.time.seconds().to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "total_amount_hacked".to_string(),
            value: amount.to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "bounty".to_string(),
            value: bounty.to_string(),
        },
        Trait {
            display_type: None,
            trait_type: "hacker_addr".to_string(),
            value: hacker_addr.to_string(),
        },
    ];

    // mint a new NFT with hack details to hacker's address as a message
    let metadata = Metadata {
        image: None,
        image_data: None,
        external_url: None,
        description: None,
        name: None,
        attributes: Some(traits),
        background_color: None,
        animation_url: None,
        youtube_url: None,
    };
    let num_tokens: NumTokensResponse = deps
        .querier
        .query_wasm_smart(cw721_addr.clone(), &Cw721QueryMsg::NumTokens {})?;
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw721_addr.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
            token_id: (num_tokens.count + 1).to_string(),
            owner: hacker_addr.to_string(),
            token_uri: None,
            extension: Some(metadata),
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
    cw20_addr: String,
    amount: u128,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }
    if amount == 0 {
        return Err(ContractError::NothingToWithdraw {});
    }
    let recipient = deps
        .api
        .addr_validate(&recipient.unwrap_or(config.contract_owner.to_string()))?;

    // send cw20 tokens to recipient as a message
    let send_msg = Cw20ExecuteMsg::Transfer {
        recipient: recipient.to_string(),
        amount: Uint128::from(amount),
    };
    let msg = WasmMsg::Execute {
        contract_addr: cw20_addr,
        msg: to_binary(&send_msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount.to_string())
        .add_message(msg))
}

pub fn update_subscription(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_bounty_pct: Option<u16>,
    new_min_bounty: Option<u128>,
) -> Result<Response, ContractError> {
    let subscriptions = SUBSCRIPTIONS.load(deps.storage, info.sender.clone())?;

    if new_bounty_pct.is_none() && new_min_bounty == subscriptions.min_bounty {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_bounty_pct > Some(100) {
        return Err(ContractError::InvalidBountyPercentage {});
    }

    let subscriptions = Subscriptions {
        subscriber: subscriptions.subscriber,
        bounty_pct: new_bounty_pct.unwrap_or(subscriptions.bounty_pct),
        min_bounty: new_min_bounty,
    };
    SUBSCRIPTIONS.save(deps.storage, info.sender, &subscriptions)?;

    Ok(Response::new().add_attribute("action", "update_subscription"))
}

pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_contract_owner: Option<String>,
    new_protocol_fee: Option<u16>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    if new_contract_owner.is_none() && new_protocol_fee.is_none() {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_contract_owner == Some(config.contract_owner.to_string())
        && new_protocol_fee == Some(config.protocol_fee)
    {
        return Err(ContractError::NothingToUpdate {});
    }

    if new_protocol_fee > Some(100) {
        return Err(ContractError::InvalidProtocolFee {});
    }

    let val_new_contract_owner = deps
        .api
        .addr_validate(&new_contract_owner.unwrap_or(config.contract_owner.to_string()));

    let config = Config {
        contract_owner: val_new_contract_owner?,
        protocol_fee: new_protocol_fee.unwrap_or(config.protocol_fee),
        cw721_addr: config.cw721_addr,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}
