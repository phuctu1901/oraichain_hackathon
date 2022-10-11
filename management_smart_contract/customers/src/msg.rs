use cosmwasm_std::{HumanAddr, CanonicalAddr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub description: String,
    pub version: String,
    pub owner: HumanAddr,
    pub admins: Vec<HumanAddr>,

    pub mutable: bool,
}

/// This is like Cw721HandleMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    AddCustomerRequest(CustomerRequestMsg),
    
    Freeze {},
    /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
    /// and only works if the contract is mutable
    UpdateAdmins { admins: Vec<HumanAddr> },

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustomerRequestMsg {
    pub user_id: String,
    pub ai_service_id: String,
    pub input_data: String,
    pub ai_output_data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ContractInfo {},
    CustomerRequest {
        input_data: String,
    },
    AllTransactions {
        user_id: String,
    },
    AdminList {},
    CanExecute { sender: String },
}

/// Shows who can mint these tokens
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterResponse {
    pub minter: HumanAddr,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]

pub struct TransactionsResponse {
    pub transactions: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ContractInfoResponse {
        pub name: String,
        pub description: String,
        pub version: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AdminListResponse {
    pub admins: Vec<HumanAddr>,
    pub mutable: bool,
}
