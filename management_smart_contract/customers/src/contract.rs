use crate::helper::{map_canonical, map_human};
use crate::msg::{TransactionsResponse, ContractInfoResponse, AdminListResponse};


use cosmwasm_std::{
    attr, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse,
    MessageInfo, Order, Querier, StdError, StdResult, Storage, CanonicalAddr, HumanAddr,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg, CustomerRequestMsg};


use crate::state::{
    CONTRACT_INFO, TransactionInfo, transactions, ADMIN_LIST, AdminList
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

    let cfg = AdminList {
        admins: map_canonical(&deps.api, &msg.admins)?,
        mutable: msg.mutable,
    };
    ADMIN_LIST.save(&mut deps.storage, &cfg)?;
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
        HandleMsg::Freeze {} => handle_freeze(deps, info),
        HandleMsg::UpdateAdmins { admins } => handle_update_admins(deps, admins, info),    
    }
}


pub fn handle_add_customer_request<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    _info: MessageInfo,
    msg: CustomerRequestMsg,
) -> Result<HandleResponse, ContractError> {
    // let minter = MINTER.load(&deps.storage)?;
    let sender_raw = deps.api.canonical_address(&_info.sender)?;

    let cfg = ADMIN_LIST.load(&deps.storage)?;
    let can = cfg.is_admin(sender_raw.to_string());
    if !can {
        return Err(ContractError::Unauthorized {});
    }
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


pub fn handle_freeze<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _info: MessageInfo,
) -> Result<HandleResponse, ContractError>  {
    let sender_raw = deps.api.canonical_address(&_info.sender)?;

    let mut cfg = ADMIN_LIST.load(&deps.storage)?;
    let can = cfg.is_admin(sender_raw.to_string());
    if !can {
        return Err(ContractError::Unauthorized {});
    } else {
        cfg.mutable = false;
        ADMIN_LIST.save(&mut deps.storage, &cfg)?;

        let mut res = HandleResponse::default();
        // res.log = vec![log("action", "freeze")];
        Ok(res)
    }
}

pub fn handle_update_admins<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    admins: Vec<HumanAddr>,
    _info: MessageInfo,
) -> Result<HandleResponse, ContractError>  {
    let sender_raw = deps.api.canonical_address(&_info.sender)?;

    let mut cfg = ADMIN_LIST.load(&deps.storage)?;
    let can = cfg.is_admin(sender_raw.to_string());
    if !can {
        return Err(ContractError::Unauthorized {});
    } else {
        cfg.admins = map_canonical(&deps.api, &admins)?;
        ADMIN_LIST.save(&mut deps.storage, &cfg)?;


        let mut res = HandleResponse::default();
        // res.log = vec![log("action", "update_admins")];
        Ok(res)
    }
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
        QueryMsg::AdminList {  } => to_binary(&query_admin_list(deps)?),
        QueryMsg::CanExecute { sender } => todo!(),
    }
}

fn query_contract_info<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<ContractInfoResponse> {
    CONTRACT_INFO.load(&deps.storage)
}

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

// pub fn query_admin_list<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,) -> StdResult<AdminListResponse> {
//     let cfg = ADMIN_LIST.load(&deps.storage)?;
//     Ok(AdminListResponse {
//         admins: cfg.admins.into_iter().map(|a| a.to_string()).collect(),
//         mutable: cfg.mutable,
//     })
// }

pub fn query_admin_list<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<AdminListResponse> {
    let cfg = ADMIN_LIST.load(&deps.storage)?;
    Ok(AdminListResponse {
        admins: map_human(&deps.api, &cfg.admins)?,
        mutable: cfg.mutable,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::CosmosMsg;
    use cosmwasm_std::WasmMsg;

    use super::*;

    const MINTER: &str = "merlin";
    const CONTRACT_NAME: &str = "Magic Power";
    const SYMBOL: &str = "MGK";
    const OWNER: &str = "orai17jfg0q25wzqqr46cpuwvhksakgxhgmf0xsqjw5";

    // fn setup_contract<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>) {
    //     let msg = InitMsg {
    //         name: CONTRACT_NAME.to_string(),
    //         description: CONTRACT_NAME.to_string(),
    //         version: CONTRACT_NAME.to_string(),
    //         owner: cosmwasm_std::HumanAddr(OWNER.to_owned()),
    //         admins: todo!(),
    //         mutable: todo!(),
           
    //     };
    //     let info = mock_info("creator", &[]);
    //     let res = init(deps, mock_env(), info, msg).unwrap();
    //     assert_eq!(0, res.messages.len());
    // }
}
