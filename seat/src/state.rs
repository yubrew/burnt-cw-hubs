use std::{cell::RefCell, rc::Rc};

use burnt_glue::module::Module;
use cosmwasm_std::{Addr, DepsMut, Empty, Env, MessageInfo, Response, Uint64};
use cw_storage_plus::{Item, Map};
use ownable::Ownable;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use token::Tokens;

use crate::{msg::InstantiateMsg, ContractError};

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
    pub ownable: Ownable<'a>,
    pub metadata: metadata::Metadata<'a, T>,
    pub seat_token: Tokens<'a, U, Empty, Empty, Empty>,
    pub redeemable: redeemable::Redeemable<'a>,
    pub sellable_token: sellable::Sellable<'a, U, Empty, Empty, Empty>,
}

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
            Some("burnt".to_string()),
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

        SeatModules {
            ownable: borrowable_ownable.take(),
            metadata,
            seat_token: borrowable_seat_token.take(),
            redeemable,
            sellable_token,
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
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.ownable)
            .map_err(|err| ContractError::OwnableError(err))?;

        // metadata module
        self.metadata
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.metadata)
            .map_err(|err| ContractError::MetadataError(err))?;

        // Burnt token module
        self.seat_token
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.seat_token)
            .map_err(|err| ContractError::SeatTokenError(err))?;

        // Redeemable token
        self.redeemable
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.redeemable)
            .map_err(|err| ContractError::RedeemableError(err))?;

        // Sellable token
        if let Some(sellable_items) = msg.sellable {
            self.sellable_token
                .instantiate(&mut mut_deps.branch(), &env, &info, sellable_items)
                .map_err(|err| ContractError::SellableError(err))?;
        } else {
            self.sellable_token
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
}
pub const CONFIG: Item<Config> = Item::new("config");
pub const CONTRACT_INFO: Item<ContractVersion> = Item::new("contract_info");
