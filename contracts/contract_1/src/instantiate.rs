use crate::{
    msg::InstantiateMsg,
    state::{Config, CONFIG},
    ContractError,
};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult};
use cw2::set_contract_version;
use cw721::{ContractInfoResponse, Cw721QueryMsg};
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:boilerplate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };

    let cfg = Config {
        contract_owner: deps.api.addr_validate(&msg.minter)?,
        protocol_fee_bps: msg.protocol_fee_bps,
        cw721_contract_addr: Addr::unchecked(""),
        cw721_code_id: msg.cw721_code_id,
    };
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

// Q: when is this triggered??
pub fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
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
