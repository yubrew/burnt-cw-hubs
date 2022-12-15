use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub modules: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub struct MigrateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum QueryMsg {
    Query(String),
}
