#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::gamm::v1beta1::{QueryPoolRequest, QueryPoolResponse};

use crate::error::ContractError;
use crate::msg::{InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-osmosis";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const QUERY_POOL_ID: u64 = 1; // TODO figure this out

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn execute(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> Result<Response, ContractError> {
//     // match msg {}
// }

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Query(QueryPoolRequest { pool_id }) => to_binary(&query_pool(deps, pool_id)?),
    }
}

fn query_pool(deps: Deps, pool_id: u64) -> StdResult<QueryPoolResponse> {
    // Ok(GetCountResponse { count: state.count })

    let submessage = SubMsg::reply_on_success(QueryPoolRequest { pool_id }, QUERY_POOL_ID);

    Ok(Response::new().add_submessage(submessage))
}
