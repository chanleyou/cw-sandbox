// use cosmwasm_std::SubMsg;
use osmosis_std::types::osmosis::gamm::v1beta1::QueryPoolRequest;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    Query(QueryPoolRequest),
}
