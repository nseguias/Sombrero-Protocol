#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{execute, instantiate, query};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    instantiate::instantiate(deps, _env, info, _msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Boilerplate {} => execute::boilerplate(deps, _env, info),
        ExecuteMsg::UpdateConfig {
            new_contract_owner,
            new_protocol_fee_bps,
        } => execute::update_config(deps, _env, info, new_contract_owner, new_protocol_fee_bps),
        ExecuteMsg::Subscribe {
            beneficiary,
            commission_bps,
            balance: u128,
        } => execute::subscribe(deps, _env, info, beneficiary, commission_bps, balance),
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, msg),
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
