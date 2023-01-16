#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use crate::error::ContractError;
use crate::instantiate::handle_instantiate_reply;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{execute, instantiate, query};

const CONTRACT_NAME: &str = "crates.io:white-hat-hacker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

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

// Q: messed up the error types here
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Boilerplate {} => execute::boilerplate(deps, env, info),
        ExecuteMsg::UpdateConfig {
            new_contract_owner,
            new_protocol_fee_bps,
        } => execute::update_config(deps, env, info, new_contract_owner, new_protocol_fee_bps),
        ExecuteMsg::Subscribe {
            subscribe_contract,
            commission_bps,
        } => execute::subscribe(deps, env, info, commission_bps, subscribe_contract),
        ExecuteMsg::Receive { cw20_msg } => execute::handle_receive_cw20(deps, env, info, cw20_msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    reply: Reply,
) -> Result<Response, ContractError> {
    match reply.id {
        INSTANTIATE_CW721_REPLY_ID => handle_instantiate_reply(deps, reply),
        id => Err(ContractError::UnknownReplyId { id }),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Boilerplate {} => to_binary(&query::boilerplate(deps)?),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}
