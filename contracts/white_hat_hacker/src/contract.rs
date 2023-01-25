#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use crate::error::ContractError;
use crate::instantiate::handle_cw721_instantiate_reply;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{execute, instantiate, query};

const CONTRACT_NAME: &str = "crates.io:white-hat-hacker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INSTANTIATE_CW721_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    instantiate::instantiate(
        deps,
        _env,
        info,
        _msg,
        CONTRACT_NAME,
        CONTRACT_VERSION,
        INSTANTIATE_CW721_REPLY_ID,
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            new_contract_owner,
            new_bounty_pct,
        } => execute::update_config(deps, env, info, new_contract_owner, new_bounty_pct),
        ExecuteMsg::Subscribe {
            subscriber,
            bounty_pct,
            min_bounty,
        } => execute::subscribe(deps, env, info, subscriber, bounty_pct, min_bounty),
        ExecuteMsg::Unsubscribe {} => execute::unsubscribe(deps, env, info),
        ExecuteMsg::Receive(cw20_msg) => execute::handle_receive_cw20(deps, env, info, cw20_msg),
        ExecuteMsg::Withdraw {
            cw20_addr,
            amount,
            recipient,
        } => execute::withdraw(deps, env, info, cw20_addr, amount, recipient),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    // _info: MessageInfo,
    reply: Reply,
) -> Result<Response, ContractError> {
    match reply.id {
        INSTANTIATE_CW721_REPLY_ID => handle_cw721_instantiate_reply(deps, reply),
        id => Err(ContractError::UnknownReplyId { id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Subscriber { protected_addr } => {
            to_binary(&query::subscriber(deps, protected_addr)?)
        }
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
