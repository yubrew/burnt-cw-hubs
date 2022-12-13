#[cfg(not(feature = "library"))]
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{entry_point, from_slice, to_vec};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use cw2::set_contract_version;
use semver::Version;

use crate::error::ContractError;
use crate::manager::contract_manager::get_manager;
use crate::state::Config;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:seat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct MigrateMsg {
    pub owner: String,
}

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
    manager.instantiate(deps, env, info, msg.as_str())
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
    use crate::state::{SeatMetadata, TokenMetadata};

    use super::*;
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env, mock_info},
        Coin, Empty, Uint64,
    };
    use cw721::{Cw721QueryMsg, NumTokensResponse, TokensResponse};
    use cw721_base::{ExecuteMsg, MintMsg, QueryMsg};
    use metadata::QueryResp as MetadataQueryResp;
    use redeemable::{
        ExecuteMsg as RedeemableExecuteMsg, QueryMsg as RedeemableQueryMsg,
        QueryResp as RedeemableQueryResp,
    };
    use schemars::{Map, Set};
    use sellable::msg::{
        ExecuteMsg as SellableExecuteMsg, QueryMsg as SellableQueryMsg,
        QueryResp as SellableQueryResp,
    };
    use serde_json::json;
    use token::QueryResp as TokenQueryResp;

    const CREATOR: &str = "cosmos188rjfzzrdxlus60zgnrvs4rg0l73hct3azv93z";

    #[test]
    fn test_seat_module_instantiation() {
        let mut deps = mock_dependencies();
        let metadata_msg = SeatMetadata {
            name: "Kenny's contract".to_string(),
        };
        let msg = json!({
            "seat_token": {
                "name": "Kenny's Token Contract".to_string(),
                "symbol": "KNY".to_string(),
                "minter": CREATOR.to_string(),
            },
            "metadata": {
                "metadata": metadata_msg
            }
        })
        .to_string();
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
                assert_eq!(meta, metadata_msg);
            }
        }

        let query_msg = QueryMsg::<Cw721QueryMsg>::NumTokens {};
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "seat_token": query_msg }).to_string(),
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
    fn test_seat_module_tokens() {
        let mut deps = mock_dependencies();
        let msg = json!({
            "seat_token": {
                "name": "Kenny's contract".to_string(),
                "symbol": "KNY".to_string(),
                "minter": CREATOR.to_string(),
            },
            "ownable": {"owner": CREATOR},
            "redeemable": {"locked_items": Set::<String>::new()},
        })
        .to_string();
        let env = mock_env();
        let info = mock_info(CREATOR, &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // mint a token
        for token_id in vec!["1", "2"] {
            let msg = ExecuteMsg::<TokenMetadata, Empty>::Mint(MintMsg {
                token_id: token_id.to_string(),
                owner: CREATOR.to_string(),
                token_uri: Some("https://example.com".to_string()),
                extension: TokenMetadata {
                    name: Some("".to_string()),
                    description: Some("".to_string()),
                    royalty_percentage: Some(0),
                    royalty_payment_address: Some("".to_string()),
                },
            });
            let mint_msg = json!({ "seat_token": msg }).to_string();

            execute(&mut deps.as_mut(), env.clone(), info.clone(), mint_msg).unwrap();
        }

        let query_msg = QueryMsg::<Cw721QueryMsg>::NumTokens {};
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "seat_token": query_msg }).to_string(),
        )
        .unwrap();
        let result: TokenQueryResp = from_binary(&res).unwrap();
        match result {
            TokenQueryResp::Result(res) => {
                let token_count: NumTokensResponse = from_binary(&res).unwrap();
                assert_eq!(token_count.count, 2);
            }
        }

        // Get all listed tokens
        let query_msg = SellableQueryMsg::ListedTokens {
            start_after: None,
            limit: None,
        };
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "sellable_token": query_msg }).to_string(),
        )
        .unwrap();
        let result: SellableQueryResp<TokenMetadata> = from_binary(&res).unwrap();
        match result {
            SellableQueryResp::ListedTokens(res) => {
                assert_eq!(res.len(), 0);
            }
        }
        // List the token
        let msg = SellableExecuteMsg::List {
            listings: Map::from([
                ("1".to_string(), Uint64::new(200)),
                ("2".to_string(), Uint64::new(100)),
            ]),
        };
        let list_msg = json!({ "sellable_token": msg }).to_string();
        execute(&mut deps.as_mut(), env.clone(), info.clone(), list_msg).unwrap();
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "sellable_token": query_msg }).to_string(),
        )
        .unwrap();
        let result: SellableQueryResp<TokenMetadata> = from_binary(&res).unwrap();
        match result {
            SellableQueryResp::ListedTokens(res) => {
                assert_eq!(res.len(), 2);
            }
        }
        // buy a token
        let msg = SellableExecuteMsg::Buy {};
        let buy_msg = json!({ "sellable_token": msg }).to_string();
        let buyer_info = mock_info("buyer", &[Coin::new(200, "burnt")]);
        execute(&mut deps.as_mut(), env.clone(), buyer_info, buy_msg).unwrap();
        // Get all listed tokens
        let query_msg = SellableQueryMsg::ListedTokens {
            start_after: None,
            limit: None,
        };
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "sellable_token": query_msg }).to_string(),
        )
        .unwrap();
        let result: SellableQueryResp<TokenMetadata> = from_binary(&res).unwrap();
        match result {
            SellableQueryResp::ListedTokens(res) => {
                assert_eq!(res.len(), 1);
                let (token_id, price, _) = &res[0];
                assert_eq!(token_id, "1");
                assert_eq!(price, Uint64::new(200));
            }
        }
        // Lock the token
        let msg = RedeemableExecuteMsg::RedeemItem("1".to_string());
        let lock_msg = json!({ "redeemable": msg }).to_string();

        execute(
            &mut deps.as_mut(),
            env.clone(),
            info.clone(),
            lock_msg.clone(),
        )
        .unwrap();
        // Confirm the token is locked
        let query_msg = RedeemableQueryMsg::IsRedeemed("1".to_string());
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "redeemable": query_msg }).to_string(),
        );
        let result: RedeemableQueryResp = from_binary(&res.unwrap()).unwrap();
        match result {
            RedeemableQueryResp::IsRedeemed(res) => {
                assert_eq!(res, true);
            }
        }
        // buy a token
        let msg = SellableExecuteMsg::Buy {};
        let buy_msg = json!({ "sellable_token": msg }).to_string();
        let buyer_info = mock_info("buyer", &[Coin::new(10, "burnt")]);
        let buy_response = execute(&mut deps.as_mut(), env.clone(), buyer_info, buy_msg);
        match buy_response {
            Err(val) => {
                print!("{:?}", val);
                assert!(true)
            }
            _ => assert!(false),
        }
        // Get all buyer owned tokens
        let query_msg = QueryMsg::<Cw721QueryMsg>::Tokens {
            owner: "buyer".to_string(),
            start_after: None,
            limit: None,
        };
        let res = query(
            deps.as_ref(),
            env.clone(),
            json!({ "seat_token": query_msg }).to_string(),
        );
        let result: TokenQueryResp = from_binary(&res.unwrap()).unwrap();
        match result {
            TokenQueryResp::Result(res) => {
                let tokens: TokensResponse = from_binary(&res).unwrap();
                assert_eq!(tokens.tokens.len(), 1);
                assert_eq!(tokens.tokens[0], "2");
            }
        }
    }
}
