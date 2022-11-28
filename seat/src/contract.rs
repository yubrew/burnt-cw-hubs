#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, from_slice, to_vec};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;
use semver::Version;

use crate::error::ContractError;
use crate::manager::contract_manager::get_manager;
use crate::msg::MigrateMsg;
use crate::state::Config;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:seat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: String,
) -> Result<Response<Binary>, String> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    // instantiate all modules
    let mut manager = get_manager();
    manager
        .instantiate(deps, env, info, msg.as_str())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: &mut DepsMut,
    env: Env,
    info: MessageInfo,
    msg: String,
) -> Result<Response, String> {
    let mut manager = get_manager();
    manager.execute(deps, env, info, msg.as_str())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: String) -> StdResult<Binary> {
    let mut manager = get_manager();
    manager.query(&deps.clone(), env, msg.as_str())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    let old_contract_ver = Version::parse(&ver.version).unwrap();
    let new_contract_ver = Version::parse(CONTRACT_VERSION).unwrap();
    // ensure we are migrating from an allowed version
    if old_contract_ver.ge(&new_contract_ver) {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    let data = deps
        .storage
        .get(b"config")
        .ok_or_else(|| StdError::not_found("State"))?;
    let mut config: Config = from_slice(&data)?;
    config.owner = deps.api.addr_validate(&msg.owner)?;
    deps.storage.set(b"config", &to_vec(&config)?);
    //set the new version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg(test)]
mod tests {

    use crate::{msg::InstantiateMsg, state::{SeatMetadata, TokenMetadata}};

    use super::*;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info}, Empty,
    };
    use token::QueryResp as TokenQueryResp;
    use metadata::QueryResp as MetadataQueryResp;
    use cw721_base::{QueryMsg, ExecuteMsg, MintMsg};
    use cw721::{NumTokensResponse, Cw721QueryMsg};
    use serde_json::json;

    const CREATOR: &str = "cosmos188rjfzzrdxlus60zgnrvs4rg0l73hct3azv93z";

    #[test]
    fn test_seat_module_instantiation() {
        let mut deps = mock_dependencies();
        let seat_token_meta = InstantiateMsg {
            name: "Kenny's Token Contract".to_string(),
            symbol: "KNY".to_string(),
            minter: CREATOR.to_string()
        };
        let metadata_msg = SeatMetadata {
            name: "Kenny's contract".to_string(), 
        };
        let msg = json!({
            "seat": seat_token_meta,
            "metadata": {
                "metadata": metadata_msg
            }
        }).to_string();
        let env = mock_env();
        let info = mock_info(CREATOR, &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // make sure seat contract metadata was created
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({"metadata": {"get_metadata": {}}}).to_string(),
        )
        .unwrap();
        let metadata: MetadataQueryResp<SeatMetadata> = from_binary(&res).unwrap();
        match metadata {
            MetadataQueryResp::Metadata(meta) => {
                assert_eq!(
                    meta,
                    metadata_msg
                );
            }
        }

        let query_msg = QueryMsg::<Cw721QueryMsg>::NumTokens {};
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({"seat": query_msg}).to_string(),
        )
        .unwrap();
        let result: TokenQueryResp = from_binary(&res).unwrap();
        match result {
            TokenQueryResp::Result(res) => {
                let token_count: NumTokensResponse = from_binary(&res).unwrap();
                assert_eq!(token_count.count, 0);
            }
        }
    }

    #[test]
    fn test_seat_module_minting() {
        let mut deps = mock_dependencies();
        let seat_meta = InstantiateMsg {
            name: "Kenny's contract".to_string(),
            symbol: "KNY".to_string(),
            minter: CREATOR.to_string()
        };
        let msg = json!({
            "seat": seat_meta
        }).to_string();
        let env = mock_env();
        let info = mock_info(CREATOR, &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // mint a token
        let msg = ExecuteMsg::<TokenMetadata, Empty>::Mint(
            MintMsg {
                token_id: "1".to_string(),
                owner: CREATOR.to_string(),
                token_uri: Some("https://example.com".to_string()),
                extension: TokenMetadata { name: Some("".to_string()), description: Some("".to_string()), royalty_percentage: Some(0), royalty_payment_address: Some("".to_string()) }

            });
        let mint_msg = json!({
            "seat": msg
        }).to_string();

        execute(
            &mut deps.as_mut(),
            env.clone(),
            info.clone(),
            mint_msg,
        ).unwrap();
        
        let query_msg = QueryMsg::<Cw721QueryMsg>::NumTokens {};
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({"seat": query_msg}).to_string(),
        )
        .unwrap();
        let result: TokenQueryResp = from_binary(&res).unwrap();
        match result {
            TokenQueryResp::Result(res) => {
                let token_count: NumTokensResponse = from_binary(&res).unwrap();
                assert_eq!(token_count.count, 1);
            }
        }
    }
}
