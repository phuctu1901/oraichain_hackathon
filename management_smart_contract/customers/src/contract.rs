use crate::msg::{TransactionsResponse, ContractInfoResponse};


use cosmwasm_std::{
    attr, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse,
    MessageInfo, Order, Querier, StdError, StdResult, Storage,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg, CustomerRequestMsg};
use crate::state::{
    CONTRACT_INFO, MINTER, TransactionInfo, transactions
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oraiconet";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    set_contract_version(&mut deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let info = ContractInfoResponse {
        name: msg.name,
        description: msg.description,
        version: msg.version
    };
    CONTRACT_INFO.save(&mut deps.storage, &info)?;
    let owner = deps.api.canonical_address(&msg.owner)?;
    MINTER.save(&mut deps.storage, &owner)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::AddCustomerRequest(msg) => handle_add_customer_request(deps, env, info, msg),
    }
}


pub fn handle_add_customer_request<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    msg: CustomerRequestMsg,
) -> Result<HandleResponse, ContractError> {
    // let minter = MINTER.load(&deps.storage)?;
    // let sender_raw = deps.api.canonical_address(&info.sender)?;

    // if sender_raw != minter {
    //     return Err(ContractError::Unauthorized {});
    // }

    let transaction = TransactionInfo {
        user_id: msg.user_id,
        ai_service_id: msg.ai_service_id,
        input_data: msg.input_data,
        ai_output_data: msg.ai_output_data,
    };
    let id = transaction.input_data.to_owned();
    transactions().update(&mut deps.storage, &id, |old| match old {
        Some(_) => Err(ContractError::Claimed {}),
        None => Ok(transaction),
    })?;    

    // increment_tokens(&mut deps.storage)?;

    Ok(HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("user_id", id),
        ],
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::ContractInfo {} => to_binary(&query_contract_info(deps)?),
        QueryMsg::CustomerRequest {input_data } => to_binary(&query_customer_request(deps, input_data)?),
        QueryMsg::AllTransactions {
            user_id,
        } => to_binary(&query_all_customer_requests(deps, user_id)?),
    }
}

fn query_contract_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ContractInfoResponse> {
    CONTRACT_INFO.load(&deps.storage)
}

// fn query_num_tokens<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
// ) -> StdResult<NumTokensResponse> {
//     let count = num_tokens(&deps.storage)?;
//     Ok(NumTokensResponse { count })
// }

fn query_customer_request<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    input_data: String,
) -> StdResult<CustomerRequestMsg> {
    let info = transactions().load(&deps.storage, &input_data)?;
    Ok(CustomerRequestMsg {
        user_id: info.user_id,
        ai_service_id: info.ai_service_id,
        input_data: info.input_data,
        ai_output_data: info.ai_output_data,
    })
}

fn query_all_customer_requests<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    user_id: String,
) -> StdResult<TransactionsResponse> {

    let transactions: Result<Vec<String>, _> = transactions::<S>()
        .idx
        .user_id
        .pks(&deps.storage, &user_id.as_bytes().to_vec(), None, None, Order::Ascending)
        .take(100)        
        .map(String::from_utf8)
        .collect();
    let transactions = transactions.map_err(StdError::invalid_utf8)?;
    Ok(TransactionsResponse { transactions })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::CosmosMsg;
    use cosmwasm_std::WasmMsg;

    use super::*;
    use cw721::ApprovedForAllResponse;

    const MINTER: &str = "merlin";
    const CONTRACT_NAME: &str = "Magic Power";
    const SYMBOL: &str = "MGK";

    // fn setup_contract<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>) {
    //     let msg = InitMsg {
    //         name: CONTRACT_NAME.to_string(),
    //         symbol: SYMBOL.to_string(),
    //         minter: MINTER.into(),
    //     };
    //     let info = mock_info("creator", &[]);
    //     let res = init(deps, mock_env(), info, msg).unwrap();
    //     assert_eq!(0, res.messages.len());
    // }
}
