#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{GoldenRResponse, InstantiateMsg, ExecuteMsg, QueryMsg, ScoreResponse};
use crate::state::{State, STORAGE};

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
        scores: vec![],
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STORAGE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("golden ratio", msg.goldenr.to_string())
        .add_attribute("scores", "".to_string())    
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpsertScore { score } => try_upsert_score(deps, info, score),
    }
}

fn try_upsert_score(
    deps: DepsMut,
    info: MessageInfo,
    score: u16,
) -> Result<Response, ContractError> {
    let mut state = STORAGE.load(deps.storage)?;
    let sender = info.sender.clone();
    let scores = &mut state.scores;
    let index = scores.iter().position(|(s, _)| s == &sender);
    match index {
        Some(i) => {
            scores[i].1 = score;
        },
        None => {
            scores.push((sender.clone(), score));
        }
    }

    STORAGE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("method", "upsert")
        .add_attribute("player", info.sender)
        .add_attribute("score", score.to_string())
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
        QueryMsg::GetScores {} => to_binary(&query_scores(deps)?),
    }
}

fn query_goldenr(deps: Deps) -> StdResult<GoldenRResponse> {
    let state = STORAGE.load(deps.storage)?;
    Ok(GoldenRResponse { goldenr: state.goldenr })

}

fn query_scores(deps: Deps) -> StdResult<ScoreResponse> {
    let state = STORAGE.load(deps.storage)?;
    Ok(ScoreResponse { scores: state.scores })
}