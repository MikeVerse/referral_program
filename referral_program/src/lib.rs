mod contract;
mod msg;

use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: msg::InstantiateMsg) -> StdResult<Response> {
    contract::instantiate(deps, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::ExecuteMsg) -> StdResult<Response> {
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(deps, msg)
}
