use crate::{
    msg::InstantiateMsg,
    state::{Config, Metadata, CONFIG},
    ContractError,
};
use cosmwasm_std::{
    to_binary, Addr, DepsMut, Empty, Env, MessageInfo, Reply, Response, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:boilerplate";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INSTANTIATE_CW721_REPLY_ID: u64 = 0;

pub type Extension = Option<Metadata>;
pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let cfg = Config {
        contract_owner: deps.api.addr_validate(&env.contract.address.to_string())?,
        protocol_fee_bps: msg.protocol_fee_bps,
        cw721_contract_addr: Addr::unchecked(""),
    };
    CONFIG.save(deps.storage, &cfg)?;

    let message = SubMsg::<Empty>::reply_on_success(
        WasmMsg::Instantiate {
            admin: msg.cw721_admin,
            code_id: msg.cw721_code_id,
            msg: to_binary(&cw721_base::msg::InstantiateMsg {
                name: msg.cw721_name,
                symbol: msg.cw721_symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            label: msg.cw721_label.to_string(),
        },
        INSTANTIATE_CW721_REPLY_ID,
    );

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        //  this should be .add_submessage(instatiate_cw721_msg)) instead
        .add_submessage(message))
}

// Q: when is this triggered??
pub fn handle_instantiate_reply(deps: DepsMut, reply: Reply) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(reply)?;
    let cw721_addr = deps.api.addr_validate(&res.contract_address)?;

    CONFIG.update(deps.storage, |mut cfg| -> Result<_, ContractError> {
        cfg.cw721_contract_addr = deps.api.addr_validate(&res.contract_address)?;
        Ok(cfg)
    })?;

    Ok(Response::new()
        .add_attribute("method", "instantiate_cw721_reply")
        .add_attribute("cw721_addr", cw721_addr))
}
