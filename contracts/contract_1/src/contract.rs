#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};

use crate::error::ContractError;
use crate::execute::handle_receive_cw20;
use crate::instantiate::handle_instantiate_reply;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{execute, instantiate, query};

pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;
pub const RECEIVE_CW20_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    instantiate::instantiate(deps, _env, info, _msg)
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
    env: Env,
    info: MessageInfo,
    msg: Reply,
) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_CW721_REPLY_ID => handle_instantiate_reply(deps, msg),
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

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
//     match reply.id {
//         INSTANTIATE_CW721_REPLY_ID => {
//             let res = parse_reply_instantiate_data(reply)?;
//             let cw721_addr = deps.api.addr_validate(&res.contract_address)?;

//             // We need to map this address back to a class
//             // ID. Fourtunately, we set the name of the new NFT
//             // contract to the class ID.
//             let cw721::ContractInfoResponse { name: class_id, .. } = deps
//                 .querier
//                 .query_wasm_smart(cw721_addr.clone(), &cw721::Cw721QueryMsg::ContractInfo {})?;

//             // Save classId <-> contract mappings.
//             CLASS_ID_TO_NFT_CONTRACT.save(deps.storage, class_id.clone(), &cw721_addr)?;
//             NFT_CONTRACT_TO_CLASS_ID.save(deps.storage, cw721_addr.clone(), &class_id)?;

//             Ok(Response::default()
//                 .add_attribute("method", "instantiate_cw721_reply")
//                 .add_attribute("class_id", class_id)
//                 .add_attribute("cw721_addr", cw721_addr))
//         }
//         INSTANTIATE_PROXY_REPLY_ID => {
//             let res = parse_reply_instantiate_data(reply)?;
//             let proxy_addr = deps.api.addr_validate(&res.contract_address)?;
//             PROXY.save(deps.storage, &Some(proxy_addr))?;

//             Ok(Response::default()
//                 .add_attribute("method", "instantiate_proxy_reply_id")
//                 .add_attribute("proxy", res.contract_address))
//         }
//         // These messages don't need to do any state changes in the
//         // reply - just need to commit an ack.
//         ACK_AND_DO_NOTHING => {
//             match reply.result {
//                 // On success, set a successful ack. Nothing else to do.
//                 SubMsgResult::Ok(_) => Ok(Response::new().set_data(ack_success())),
//                 // On error we need to use set_data to override the data field
//                 // from our caller, the IBC packet recv, and acknowledge our
//                 // failure.  As per:
//                 // https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#handling-the-reply
//                 SubMsgResult::Err(err) => Ok(Response::new().set_data(ack_fail(err))),
//             }
//         }
//         _ => Err(ContractError::UnrecognisedReplyId {}),
//     }
// }
