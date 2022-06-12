use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo, StdResult,
};

// make use of the custom errors
pub fn init(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {}
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Aggregate { results } => query_aggregation(deps, results),
    }
}

fn query_aggregation(_deps: Deps, results: Vec<String>) -> StdResult<Binary> {
    let result_bin = to_binary(&results).unwrap();
    Ok(result_bin)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    #[test]
    fn assert_aggregate() {
        let deps = mock_dependencies(&[]);
    //     let resp = format!(
    //     "[{{\"name\":\"ETH\",\"prices\":[\"{}\",\"{}\",\"{}\"]}},{{\"name\":\"BTC\",\"prices\":[\"{}\",\"{}\"]}},{{\"name\":\"LINK\",\"prices\":[\"{}\",\"{}\"]}}]",
    //     "0.00000000000018900", "0.00000001305", "0.00000000006", "2801.2341", "200.1", ".1", "44"
    // );

    let resp = format!(
        "\"39734.png\""
    );
        let mut results: Vec<String> = Vec::new();
        results.push(resp);
        let query_result = query_aggregation(deps.as_ref(), results).unwrap();
        let query_result_str = query_result.to_string();
        println!("query result str: {}", query_result_str);
    }
}
