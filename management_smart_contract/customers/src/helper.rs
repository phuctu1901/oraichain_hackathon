use cosmwasm_std::{Api, HumanAddr, CanonicalAddr, StdResult};

pub fn map_canonical<A: Api>(api: &A, admins: &[HumanAddr]) -> StdResult<Vec<CanonicalAddr>> {
    admins
        .iter()
        .map(|addr| api.canonical_address(addr))
        .collect()
}

pub fn map_human<A: Api>(api: &A, admins: &[CanonicalAddr]) -> StdResult<Vec<HumanAddr>> {
    admins.iter().map(|addr| api.human_address(addr)).collect()
}
