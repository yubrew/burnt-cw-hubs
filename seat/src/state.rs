use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use sellable::msg::SellableTrait;

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
    pub list_price: Option<Uint64>,
    pub locked: bool,
    pub redeemed: bool,
}

impl SellableTrait for TokenMetadata {
    fn get_redeemed(&self) -> bool {
        self.redeemed
    }

    fn get_locked(&self) -> bool {
        self.locked
    }

    fn get_list_price(&self) -> Option<Uint64> {
        self.list_price
    }

    fn set_list_price(&mut self, price: cosmwasm_std::Uint64) -> bool {
        self.list_price = Some(price);
        true
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const CONTRACT_INFO: Item<ContractVersion> = Item::new("contract_info");
