use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

use crate::msg::ContractInfoResponse;



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransactionInfo {
    pub user_id: String,
    pub ai_service_id: String,
    pub input_data: String,
    pub ai_output_data: String,
}

pub const CONTRACT_INFO: Item<ContractInfoResponse> = Item::new(b"nft_info");
pub const MINTER: Item<CanonicalAddr> = Item::new(b"minter");
pub const TOKEN_COUNT: Item<u64> = Item::new(b"num_tokens");

// pub const TOKENS: Map<&str, TokenInfo> = Map::new(b"tokens");

pub fn num_tokens<S: Storage>(storage: &S) -> StdResult<u64> {
    Ok(TOKEN_COUNT.may_load(storage)?.unwrap_or_default())
}

pub fn increment_tokens<S: Storage>(storage: &mut S) -> StdResult<u64> {
    let val = num_tokens(storage)? + 1;
    TOKEN_COUNT.save(storage, &val)?;
    Ok(val)
}

pub struct TransactionIndexes<'a, S: Storage> {
    pub user_id: MultiIndex<'a, S, TransactionInfo>,
}

impl<'a, S: Storage> IndexList<S, TransactionInfo> for TransactionIndexes<'a, S> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<S, TransactionInfo>> + '_> {
        let v: Vec<&dyn Index<S, TransactionInfo>> = vec![&self.user_id];
        Box::new(v.into_iter())
    }
}

// pub fn tokens<'a, S: Storage>() -> IndexedMap<'a, &'a str, TokenInfo, S, TokenIndexes<'a, S>> {
//     let indexes = TokenIndexes {
//         owner: MultiIndex::new(|d| d.owner.to_vec(), b"tokens", b"tokens__owner"),
//     };
//     IndexedMap::new(b"tokens", indexes)
// }


pub fn transactions<'a, S: Storage>() -> IndexedMap<'a, &'a str, TransactionInfo, S, TransactionIndexes<'a, S>> {
    let indexes = TransactionIndexes {
        user_id: MultiIndex::new(|d| d.user_id.as_bytes().to_vec(), b"transactions", b"transactions user_id"),
    };
    IndexedMap::new(b"transactions", indexes)
}

