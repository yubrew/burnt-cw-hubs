use std::{cell::RefCell, rc::Rc};

use burnt_glue::module::Module;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint64,
};
use cw_storage_plus::{Item, Map};
use ownable::Ownable;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use token::Tokens;

use crate::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    ContractError,
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ContractVersion {
    /// contract is a globally unique identifier for the contract.
    /// it should build off standard namespacing for whichever language it is in,
    /// and prefix it with the registry we use.
    /// For rust we prefix with `crates.io:`, to give us eg. `crates.io:cw20-base`
    pub contract: String,
    /// version is any string that this implementation knows. It may be simple counter "1", "2".
    /// or semantic version on release tags "v0.6.2", or some custom feature flag list.
    /// the only code that needs to understand the version parsing is code that knows how to
    /// migrate from the given contract (and is tied to it's implementation somehow)
    pub version: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct SeatMetadata {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TokenMetadata {
    pub description: Option<String>,
    pub name: Option<String>,
    /// This is how much the minter takes as a cut when sold
    /// royalties are owed on this token if it is Some
    pub royalty_percentage: Option<u64>,
    /// The payment address, may be different to or the same
    /// as the minter addr
    /// question: how do we validate this?
    pub royalty_payment_address: Option<String>,
}

pub struct SeatModules<'a, T, U>
where
    T: Serialize + DeserializeOwned,
    U: Serialize + DeserializeOwned + Clone,
{
    pub ownable: Rc<RefCell<Ownable<'a>>>,
    pub metadata: metadata::Metadata<'a, T>,
    pub seat_token: Rc<RefCell<Tokens<'a, U, Empty, Empty, Empty>>>,
    pub redeemable: redeemable::Redeemable<'a>,
    pub sellable_token: Rc<RefCell<sellable::Sellable<'a, U, Empty, Empty, Empty>>>,
    pub sales: sales::Sales<'a, U, Empty, Empty, Empty>,
}

pub const HUB_CONTRACT: Item<Addr> = Item::new("hub_contract");

impl<'a> Default for SeatModules<'a, SeatMetadata, TokenMetadata> {
    fn default() -> Self {
        // instantiate all modules

        // ownable module
        let ownable = ownable::Ownable::default();

        let borrowable_ownable = Rc::new(RefCell::new(ownable));
        // metadata module
        let metadata = metadata::Metadata::new(
            Item::<SeatMetadata>::new("metadata"),
            borrowable_ownable.clone(),
        );
        // Burnt token module
        let seat_token = Tokens::<TokenMetadata, Empty, Empty, Empty>::new(
            cw721_base::Cw721Contract::default(),
            Some("uturnt".to_string()),
        );
        let borrowable_seat_token = Rc::new(RefCell::new(seat_token));
        // Redeemable token
        let redeemable = redeemable::Redeemable::new(Item::new("redeemed_items"));
        // Sellable token
        let sellable_token = sellable::Sellable::new(
            borrowable_seat_token.clone(),
            borrowable_ownable.clone(),
            Map::new("listed_tokens"),
        );
        let borrowable_sellable_token = Rc::new(RefCell::new(sellable_token));
        // sales module
        let sales = sales::Sales::new(
            borrowable_sellable_token.clone(),
            Item::new("primary_sales"),
        );

        SeatModules {
            ownable: borrowable_ownable,
            metadata,
            seat_token: borrowable_seat_token,
            redeemable,
            sellable_token: borrowable_sellable_token,
            sales,
        }
    }
}

impl<'a> SeatModules<'a, SeatMetadata, TokenMetadata> {
    pub fn instantiate(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let mut mut_deps = Box::new(deps);

        // ownable module
        self.ownable
            .borrow_mut()
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.ownable)
            .map_err(|err| ContractError::OwnableError(err))?;

        // metadata module
        self.metadata
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.metadata)
            .map_err(|err| ContractError::MetadataError(err))?;

        // Burnt token module
        self.seat_token
            .borrow_mut()
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.seat_token)
            .map_err(|err| ContractError::SeatTokenError(err))?;

        // Redeemable token
        self.redeemable
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.redeemable)
            .map_err(|err| ContractError::RedeemableError(err))?;

        self.sales
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.sales)
            .map_err(|err| ContractError::SalesError(err))?;

        // Sellable token
        if let Some(sellable_items) = msg.sellable {
            self.sellable_token
                .borrow_mut()
                .instantiate(&mut mut_deps.branch(), &env, &info, sellable_items)
                .map_err(|err| ContractError::SellableError(err))?;
        } else {
            self.sellable_token
                .borrow_mut()
                .instantiate(
                    &mut mut_deps.branch(),
                    &env,
                    &info,
                    sellable::msg::InstantiateMsg {
                        tokens: schemars::Map::<String, Uint64>::new(),
                    },
                )
                .map_err(|err| ContractError::SellableError(err))?;
        }

        Ok(Response::default())
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let mut mut_deps = Box::new(deps);
        match msg {
            ExecuteMsg::Ownable(msg) => {
                self.ownable
                    .borrow_mut()
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::OwnableError(err))?;
            }
            ExecuteMsg::Metadata(msg) => {
                self.metadata
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::MetadataError(err))?;
            }
            ExecuteMsg::SeatToken(msg) => {
                self.seat_token
                    .borrow_mut()
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::SeatTokenError(err))?;
            }
            ExecuteMsg::Redeemable(msg) => {
                self.redeemable
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::RedeemableError(err))?;
            }
            ExecuteMsg::Sellable(msg) => {
                self.sellable_token
                    .borrow_mut()
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::SellableError(err))?;
            }

            ExecuteMsg::Sales(msg) => {
                self.sales
                    .execute(&mut mut_deps, env, info, msg)
                    .map_err(|err| ContractError::SalesError(err))?;
            }
        }
        Ok(Response::default())
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Ownable(msg) => {
                to_binary(&self.ownable.borrow().query(&deps, env, msg).unwrap())
            }

            QueryMsg::Metadata(msg) => to_binary(&self.metadata.query(&deps, env, msg).unwrap()),
            QueryMsg::SeatToken(msg) => {
                to_binary(&self.seat_token.borrow_mut().query(&deps, env, msg).unwrap())
            }
            QueryMsg::Redeemable(msg) => {
                to_binary(&self.redeemable.query(&deps, env, msg).unwrap())
            }
            QueryMsg::Sellable(msg) => to_binary(
                &self
                    .sellable_token
                    .borrow_mut()
                    .query(&deps, env, msg)
                    .unwrap(),
            ),
            QueryMsg::Sales(msg) => to_binary(&self.sales.query(&deps, env, msg).unwrap()),
        }
    }
}
pub const CONFIG: Item<Config> = Item::new("config");
pub const CONTRACT_INFO: Item<ContractVersion> = Item::new("contract_info");
