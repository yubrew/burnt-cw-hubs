use cosmwasm_schema::cw_serde;
use serde::{Deserialize, Serialize};

use crate::state::{HubMetadata, MetadataField};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub ownable: ownable::InstantiateMsg,
    pub metadata: metadata::InstantiateMsg<HubMetadata>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Ownable(ownable::ExecuteMsg),
    UpdateMetadata(MetadataField),
}

#[cw_serde]
pub struct MigrateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum QueryMsg {
    Ownable(ownable::QueryMsg),
    Metadata(metadata::QueryMsg),
}
