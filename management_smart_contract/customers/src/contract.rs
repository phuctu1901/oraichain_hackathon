use crate::msg::TransactionsResponse;


use cosmwasm_std::{
    attr, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse,
    MessageInfo, Order, Querier, StdError, StdResult, Storage,
};

use cw2::set_contract_version;
use cw721::{
    ContractInfoResponse,
};

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
        symbol: msg.symbol,
    };
    CONTRACT_INFO.save(&mut deps.storage, &info)?;
    let minter = &msg.minter;
    MINTER.save(&mut deps.storage, &minter)?;
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        // HandleMsg::Mint(msg) => handle_mint(deps, env, info, msg),
        HandleMsg::AddCustomerRequest(msg) => handle_add_customer_request(deps, env, info, msg),
        // HandleMsg::BattleMonster {
        //     attacker_id,
        //     defender_id,
        // } => handle_battle_monster(deps, env, info, attacker_id, defender_id),
        // HandleMsg::Approve {
        //     spender,
        //     token_id,
        //     expires,
        // } => handle_approve(deps, env, info, spender, token_id, expires),
        // HandleMsg::Revoke { spender, token_id } => {
        //     handle_revoke(deps, env, info, spender, token_id)
        // }
        // HandleMsg::ApproveAll { operator, expires } => {
        //     handle_approve_all(deps, env, info, operator, expires)
        // }
        // HandleMsg::RevokeAll { operator } => handle_revoke_all(deps, env, info, operator),
        // HandleMsg::TransferNft {
        //     recipient,
        //     token_id,
        // } => handle_transfer_nft(deps, env, info, recipient, token_id),
        // HandleMsg::SendNft {
        //     contract,
        //     token_id,
        //     msg,
        // } => handle_send_nft(deps, env, info, contract, token_id, msg),
    }
}

// pub fn handle_battle_monster<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     _env: Env,
//     _info: MessageInfo,
//     attacker_id: String,
//     defender_id: String,
// ) -> Result<HandleResponse, ContractError> {
//     let mut info_attacker_id = tokens().load(&deps.storage, &attacker_id)?;
//     let mut info_defender_id = tokens().load(&deps.storage, &defender_id)?;
//     if info_attacker_id.level >= info_defender_id.level {
//         info_attacker_id.level += 2;
//         info_defender_id.level += 1;
//     } else {
//         info_attacker_id.level += 1;
//         info_defender_id.level += 2;
//     }
//     tokens().save(&mut deps.storage, &attacker_id, &info_attacker_id)?;
//     tokens().save(&mut deps.storage, &defender_id, &info_defender_id)?;
//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "battle_monster"),
//             attr("attacker_id", attacker_id),
//             attr("defender_id", defender_id),
//         ],
//         data: None,
//     })
// }

// pub fn handle_mint<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     _env: Env,
//     info: MessageInfo,
//     msg: MintMsg,
// ) -> Result<HandleResponse, ContractError> {
//     let minter = MINTER.load(&deps.storage)?;
//     let sender_raw = deps.api.canonical_address(&info.sender)?;

//     if sender_raw != minter {
//         return Err(ContractError::Unauthorized {});
//     }

//     // create the token
//     let token = TokenInfo {
//         owner: deps.api.canonical_address(&msg.owner)?,
//         approvals: vec![],
//         name: msg.name,
//         level: msg.level,
//         description: msg.description.unwrap_or_default(),
//         image: msg.image,
//     };
//     tokens().update(&mut deps.storage, &msg.token_id, |old| match old {
//         Some(_) => Err(ContractError::Claimed {}),
//         None => Ok(token),
//     })?;

//     increment_tokens(&mut deps.storage)?;

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "mint"),
//             attr("minter", info.sender),
//             attr("token_id", msg.token_id),
//         ],
//         data: None,
//     })
// }


pub fn handle_add_customer_request<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    info: MessageInfo,
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

// pub fn handle_transfer_nft<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     info: MessageInfo,
//     recipient: HumanAddr,
//     token_id: String,
// ) -> Result<HandleResponse, ContractError> {
//     _transfer_nft(deps, &env, &info, &recipient, &token_id)?;

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "transfer_nft"),
//             attr("sender", info.sender),
//             attr("recipient", recipient),
//             attr("token_id", token_id),
//         ],
//         data: None,
//     })
// }

// pub fn handle_send_nft<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     info: MessageInfo,
//     contract: HumanAddr,
//     token_id: String,
//     msg: Option<Binary>,
// ) -> Result<HandleResponse, ContractError> {
//     // Transfer token
//     _transfer_nft(deps, &env, &info, &contract, &token_id)?;

//     let send = Cw721ReceiveMsg {
//         sender: info.sender.clone(),
//         token_id: token_id.clone(),
//         msg,
//     };

//     // Send message
//     Ok(HandleResponse {
//         messages: vec![send.into_cosmos_msg(contract.clone())?],
//         attributes: vec![
//             attr("action", "send_nft"),
//             attr("sender", info.sender),
//             attr("recipient", contract),
//             attr("token_id", token_id),
//         ],
//         data: None,
//     })
// }

// pub fn _transfer_nft<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: &Env,
//     info: &MessageInfo,
//     recipient: &HumanAddr,
//     token_id: &str,
// ) -> Result<TokenInfo, ContractError> {
//     let mut token = tokens().load(&deps.storage, &token_id)?;
//     // ensure we have permissions
//     check_can_send(&deps, env, info, &token)?;
//     // set owner and remove existing approvals
//     token.owner = deps.api.canonical_address(recipient)?;
//     token.approvals = vec![];
//     tokens().save(&mut deps.storage, &token_id, &token)?;
//     Ok(token)
// }

// pub fn handle_approve<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     info: MessageInfo,
//     spender: HumanAddr,
//     token_id: String,
//     expires: Option<Expiration>,
// ) -> Result<HandleResponse, ContractError> {
//     _update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "approve"),
//             attr("sender", info.sender),
//             attr("spender", spender),
//             attr("token_id", token_id),
//         ],
//         data: None,
//     })
// }

// pub fn handle_revoke<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     info: MessageInfo,
//     spender: HumanAddr,
//     token_id: String,
// ) -> Result<HandleResponse, ContractError> {
//     _update_approvals(deps, &env, &info, &spender, &token_id, false, None)?;

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "revoke"),
//             attr("sender", info.sender),
//             attr("spender", spender),
//             attr("token_id", token_id),
//         ],
//         data: None,
//     })
// }

// pub fn _update_approvals<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: &Env,
//     info: &MessageInfo,
//     spender: &HumanAddr,
//     token_id: &str,
//     // if add == false, remove. if add == true, remove then set with this expiration
//     add: bool,
//     expires: Option<Expiration>,
// ) -> Result<TokenInfo, ContractError> {
//     let mut token = tokens().load(&deps.storage, &token_id)?;
//     // ensure we have permissions
//     check_can_approve(&deps, env, info, &token)?;

//     // update the approval list (remove any for the same spender before adding)
//     let spender_raw = deps.api.canonical_address(&spender)?;
//     token.approvals = token
//         .approvals
//         .into_iter()
//         .filter(|apr| apr.spender != spender_raw)
//         .collect();

//     // only difference between approve and revoke
//     if add {
//         // reject expired data as invalid
//         let expires = expires.unwrap_or_default();
//         if expires.is_expired(&env.block) {
//             return Err(ContractError::Expired {});
//         }
//         let approval = Approval {
//             spender: spender_raw,
//             expires,
//         };
//         token.approvals.push(approval);
//     }

//     tokens().save(&mut deps.storage, &token_id, &token)?;

//     Ok(token)
// }

// pub fn handle_approve_all<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     env: Env,
//     info: MessageInfo,
//     operator: HumanAddr,
//     expires: Option<Expiration>,
// ) -> Result<HandleResponse, ContractError> {
//     // reject expired data as invalid
//     let expires = expires.unwrap_or_default();
//     if expires.is_expired(&env.block) {
//         return Err(ContractError::Expired {});
//     }

//     // set the operator for us
//     let sender_raw = deps.api.canonical_address(&info.sender)?;
//     let operator_raw = deps.api.canonical_address(&operator)?;
//     OPERATORS.save(&mut deps.storage, (&sender_raw, &operator_raw), &expires)?;

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "approve_all"),
//             attr("sender", info.sender),
//             attr("operator", operator),
//         ],
//         data: None,
//     })
// }

// pub fn handle_revoke_all<S: Storage, A: Api, Q: Querier>(
//     deps: &mut Extern<S, A, Q>,
//     _env: Env,
//     info: MessageInfo,
//     operator: HumanAddr,
// ) -> Result<HandleResponse, ContractError> {
//     let sender_raw = deps.api.canonical_address(&info.sender)?;
//     let operator_raw = deps.api.canonical_address(&operator)?;
//     OPERATORS.remove(&mut deps.storage, (&sender_raw, &operator_raw));

//     Ok(HandleResponse {
//         messages: vec![],
//         attributes: vec![
//             attr("action", "revoke_all"),
//             attr("sender", info.sender),
//             attr("operator", operator),
//         ],
//         data: None,
//     })
// }

// /// returns true iff the sender can execute approve or reject on the contract
// fn check_can_approve<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     env: &Env,
//     info: &MessageInfo,
//     token: &TokenInfo,
// ) -> Result<(), ContractError> {
//     // owner can approve
//     let sender_raw = deps.api.canonical_address(&info.sender)?;
//     if token.owner == sender_raw {
//         return Ok(());
//     }
//     // operator can approve
//     let op = OPERATORS.may_load(&deps.storage, (&token.owner, &sender_raw))?;
//     match op {
//         Some(ex) => {
//             if ex.is_expired(&env.block) {
//                 Err(ContractError::Unauthorized {})
//             } else {
//                 Ok(())
//             }
//         }
//         None => Err(ContractError::Unauthorized {}),
//     }
// }

// /// returns true iff the sender can transfer ownership of the token
// fn check_can_send<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     env: &Env,
//     info: &MessageInfo,
//     token: &TokenInfo,
// ) -> Result<(), ContractError> {
//     // owner can send
//     let sender_raw = deps.api.canonical_address(&info.sender)?;
//     if token.owner == sender_raw {
//         return Ok(());
//     }

//     // any non-expired token approval can send
//     if token
//         .approvals
//         .iter()
//         .any(|apr| apr.spender == sender_raw && !apr.expires.is_expired(&env.block))
//     {
//         return Ok(());
//     }

//     // operator can send
//     let op = OPERATORS.may_load(&deps.storage, (&token.owner, &sender_raw))?;
//     match op {
//         Some(ex) => {
//             if ex.is_expired(&env.block) {
//                 Err(ContractError::Unauthorized {})
//             } else {
//                 Ok(())
//             }
//         }
//         None => Err(ContractError::Unauthorized {}),
//     }
// }

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        // QueryMsg::Minter {} => to_binary(&query_minter(deps)?),
        QueryMsg::ContractInfo {} => to_binary(&query_contract_info(deps)?),
        QueryMsg::CustomerRequest {input_data } => to_binary(&query_customer_request(deps, input_data)?),
        // QueryMsg::OwnerOf {
        //     token_id,
        //     include_expired,
        // } => to_binary(&query_owner_of(
        //     deps,
        //     env,
        //     token_id,
        //     include_expired.unwrap_or(false),
        // )?),
        // QueryMsg::AllNftInfo {
        //     token_id,
        //     include_expired,
        // } => to_binary(&query_all_nft_info(
        //     deps,
        //     env,
        //     token_id,
        //     include_expired.unwrap_or(false),
        // )?),
        // QueryMsg::ApprovedForAll {
        //     owner,
        //     include_expired,
        //     start_after,
        //     limit,
        // } => to_binary(&query_all_approvals(
        //     deps,
        //     env,
        //     owner,
        //     include_expired.unwrap_or(false),
        //     start_after,
        //     limit,
        // )?),
        // QueryMsg::NumTokens {} => to_binary(&query_num_tokens(deps)?),
        QueryMsg::AllTransactions {
            user_id,
        } => to_binary(&query_all_customer_requests(deps, user_id)?),
        // QueryMsg::AllTokens { start_after, limit } => {
        //     to_binary(&query_all_tokens(deps, start_after, limit)?)
        // }
        // QueryMsg::AllTransactions {  } => todo!(),
    }
}

// fn query_minter<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
// ) -> StdResult<MinterResponse> {
//     let minter_raw = MINTER.load(&deps.storage)?;
//     let minter = deps.api.human_address(&minter_raw)?;
//     Ok(MinterResponse { minter })
// }

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

// let tokens: Result<Vec<String>, _> = tokens::<S>()
// //         .idx
// //         .owner
// //         .pks(&deps.storage, &owner_raw, start, None, Order::Ascending)
// //         .take(limit)
// //         .map(String::from_utf8)
// //         .collect();

// pub user_id: String,
//     pub ai_service_id: String,
//     pub input_data: String,
//     pub ai_output_data: String,

// fn query_owner_of<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     env: Env,
//     token_id: String,
//     include_expired: bool,
// ) -> StdResult<OwnerOfResponse> {
//     let info = tokens().load(&deps.storage, &token_id)?;
//     Ok(OwnerOfResponse {
//         owner: deps.api.human_address(&info.owner)?,
//         approvals: humanize_approvals(deps.api, &env.block, &info, include_expired)?,
//     })
// }

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

// fn query_all_approvals<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     env: Env,
//     owner: HumanAddr,
//     include_expired: bool,
//     start_after: Option<HumanAddr>,
//     limit: Option<u32>,
// ) -> StdResult<ApprovedForAllResponse> {
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     let start_canon = maybe_canonical(deps.api, start_after)?;
//     let start = start_canon.map(Bound::exclusive);

//     let owner_raw = deps.api.canonical_address(&owner)?;
//     let res: StdResult<Vec<_>> = OPERATORS
//         .prefix(&owner_raw)
//         .range(&deps.storage, start, None, Order::Ascending)
//         .filter(|r| include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block))
//         .take(limit)
//         .map(|item| parse_approval(deps.api, item))
//         .collect();
//     Ok(ApprovedForAllResponse { operators: res? })
// }

// fn parse_approval<A: Api>(api: A, item: StdResult<KV<Expiration>>) -> StdResult<cw721::Approval> {
//     item.and_then(|(k, expires)| {
//         let spender = api.human_address(&k.into())?;
//         Ok(cw721::Approval { spender, expires })
//     })
// }

// fn query_tokens<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     owner: HumanAddr,
//     start_after: Option<String>,
//     limit: Option<u32>,
// ) -> StdResult<TokensResponse> {
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     let start = start_after.map(Bound::exclusive);

//     let owner_raw = deps.api.canonical_address(&owner)?;
//     let tokens: Result<Vec<String>, _> = tokens::<S>()
//         .idx
//         .owner
//         .pks(&deps.storage, &owner_raw, start, None, Order::Ascending)
//         .take(limit)
//         .map(String::from_utf8)
//         .collect();
//     let tokens = tokens.map_err(StdError::invalid_utf8)?;
//     Ok(TokensResponse { tokens })
// }

// fn query_all_tokens<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     start_after: Option<String>,
//     limit: Option<u32>,
// ) -> StdResult<TokensResponse> {
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
//     let start = start_after.map(Bound::exclusive);

//     let tokens: StdResult<Vec<String>> = tokens::<S>()
//         .range(&deps.storage, start, None, Order::Ascending)
//         .take(limit)
//         .map(|item| item.map(|(k, _)| String::from_utf8_lossy(&k).to_string()))
//         .collect();
  
//         Ok(TokensResponse { tokens: tokens? })
//     }
    

// fn query_all_nft_info<S: Storage, A: Api, Q: Querier>(
//     deps: &Extern<S, A, Q>,
//     env: Env,
//     token_id: String,
//     include_expired: bool,
// ) -> StdResult<AllNftInfoResponse> {
//     let info = tokens().load(&deps.storage, &token_id)?;
//     Ok(AllNftInfoResponse {
//         access: OwnerOfResponse {
//             owner: deps.api.human_address(&info.owner)?,
//             approvals: humanize_approvals(deps.api, &env.block, &info, include_expired)?,
//         },
//         info: NftInfoResponse {
//             name: info.name,
//             level: info.level,
//             description: info.description,
//             image: info.image,
//         },
//     })
// }

// fn humanize_approvals<A: Api>(
//     api: A,
//     block: &BlockInfo,
//     info: &TokenInfo,
//     include_expired: bool,
// ) -> StdResult<Vec<cw721::Approval>> {
//     let iter = info.approvals.iter();
//     iter.filter(|apr| include_expired || !apr.expires.is_expired(block))
//         .map(|apr| humanize_approval(api, apr))
//         .collect()
// }

// fn humanize_approval<A: Api>(api: A, approval: &Approval) -> StdResult<cw721::Approval> {
//     Ok(cw721::Approval {
//         spender: api.human_address(&approval.spender)?,
//         expires: approval.expires,
//     })
// }

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

    fn setup_contract<S: Storage, A: Api, Q: Querier>(deps: &mut Extern<S, A, Q>) {
        let msg = InitMsg {
            name: CONTRACT_NAME.to_string(),
            symbol: SYMBOL.to_string(),
            minter: MINTER.into(),
        };
        let info = mock_info("creator", &[]);
        let res = init(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }
}
