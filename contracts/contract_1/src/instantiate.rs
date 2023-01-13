use crate::{
    msg::InstantiateMsg,
    state::{Config, CONFIG},
    ContractError,
};
use cosmwasm_std::{
    to_binary, Addr, DepsMut, Env, MessageInfo, Reply, Response, StdError, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721::{ContractInfoResponse, Cw721QueryMsg};
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:boilerplate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        contract_owner: deps.api.addr_validate(&env.contract.address.to_string())?,
        protocol_fee_bps: msg.protocol_fee_bps,
        cw721_contract_addr: Addr::unchecked(""),
    };
    CONFIG.save(deps.storage, &cfg)?;

    let instatiate_cw721_msg = WasmMsg::Instantiate {
        admin: msg.cw721_admin,
        code_id: msg.cw721_code_id,
        msg: to_binary(&cw721_base::msg::InstantiateMsg {
            name: msg.cw721_name,
            symbol: msg.cw721_symbol,
            // Q: do I need to validate this address?
            minter: env.contract.address.to_string(),
        })?,
        funds: vec![],
        label: msg.cw721_label,
    };

    // Creating a submessage that wraps the message above
    let instatiate_cw721_submessage =
        SubMsg::reply_on_success(instatiate_cw721_msg, INSTANTIATE_CW721_REPLY_ID);

    // call the other instantiate cw721 contract as submessage (reply on success)
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        //  this should be .add_submessage(instatiate_cw721_msg)) instead
        .add_submessage(instatiate_cw721_submessage))
}

// fn instantiate_cw721(env: &Env, code_id: u64, game_instance_addr: String) -> WasmMsg {
//     WasmMsg::Instantiate {
//         code_id,
//         msg: to_binary(&cw721_base::msg::InstantiateMsg {
//             name: game_instance_addr.clone(),
//             symbol: "MTX".to_string(),
//             minter: game_instance_addr,
//         })
//         .unwrap(),
//         funds: vec![],
//         label: "Morra Ticket".to_string(),
//         admin: Some(env.contract.address.to_string()),
//     }
// }

// Q: when is this triggered??
pub fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg)
        .map_err(|_| StdError::generic_err("error parsing instantiate reply"))?;
    let contract_info: ContractInfoResponse = deps.querier.query_wasm_smart(
        res.contract_address.clone(),
        &Cw721QueryMsg::ContractInfo {},
    )?;
    let addr = deps.api.addr_validate(&res.contract_address)?;

    let config = CONFIG.load(deps.storage)?;
    config.cw721_contract_addr = addr;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new())
}
