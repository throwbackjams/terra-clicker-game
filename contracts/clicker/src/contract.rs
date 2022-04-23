#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{GoldenRResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = "crates.io:clicker";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    let state = State {
        goldenr: msg.goldenr,
        owner: info.sender.clone(),
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("golden ratio", msg.goldenr.to_string())    
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGoldenR {} => to_binary(&query_goldenr(deps)?),
    }
}

fn query_goldenr(deps: Deps) -> StdResult<GoldenRResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GoldenRResponse { goldenr: state.goldenr })

}