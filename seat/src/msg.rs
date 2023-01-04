use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use serde::{Deserialize, Serialize};

use crate::state::{SeatMetadata, TokenMetadata};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub ownable: ownable::InstantiateMsg,
    pub metadata: metadata::InstantiateMsg<SeatMetadata>,
    pub seat_token: cw721_base::InstantiateMsg,
    pub redeemable: redeemable::InstantiateMsg,
    // This is optional because of serde serialization error on maps
    pub sellable: Option<sellable::msg::InstantiateMsg>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Ownable(ownable::ExecuteMsg),
    Metadata(metadata::ExecuteMsg<SeatMetadata>),
    SeatToken(cw721_base::ExecuteMsg<TokenMetadata, Empty>),
    Redeemable(redeemable::ExecuteMsg),
    Sellable(sellable::msg::ExecuteMsg),
}

#[cw_serde]
pub struct MigrateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum QueryMsg {
    Ownable(ownable::QueryMsg),
    Metadata(metadata::QueryMsg),
    SeatToken(cw721_base::QueryMsg<Empty>),
    Redeemable(redeemable::QueryMsg),
    Sellable(sellable::msg::QueryMsg),
}
