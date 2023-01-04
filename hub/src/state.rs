use std::{cell::RefCell, rc::Rc};

use burnt_glue::module::Module;
use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;
use ownable::Ownable;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    msg::{InstantiateMsg, QueryMsg},
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SocialLinks {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct HubMetadata {
    pub name: String,
    pub hub_url: String,
    pub description: String,
    pub tags: Vec<String>,
    pub social_links: Vec<SocialLinks>,
    pub creator: String,
    pub image_url: String,
}

pub struct HubModules<'a, T>
where
    T: Serialize + DeserializeOwned,
{
    pub ownable: Ownable<'a>,
    pub metadata: metadata::Metadata<'a, T>,
}

impl<'a> Default for HubModules<'a, HubMetadata> {
    fn default() -> Self {
        let ownable = ownable::Ownable::default();
        let borrowable_ownable = Rc::new(RefCell::new(ownable));

        let metadata = metadata::Metadata::new(
            Item::<HubMetadata>::new("metadata"),
            borrowable_ownable.clone(),
        );

        HubModules {
            ownable: borrowable_ownable.take(),
            metadata,
        }
    }
}

impl<'a> HubModules<'a, HubMetadata> {
    pub fn new(
        ownable_module: Ownable<'a>,
        metadata_module: metadata::Metadata<'a, HubMetadata>,
    ) -> Self {
        HubModules {
            ownable: ownable_module,
            metadata: metadata_module,
        }
    }
    pub fn instantiate(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // Instantiate all modules
        let mut mut_deps = Box::new(deps);

        self.ownable
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.ownable)
            .map_err(|err| ContractError::OwnableError(err))?;

        self.metadata
            .instantiate(&mut mut_deps.branch(), &env, &info, msg.metadata)
            .map_err(|err| ContractError::MetadataError(err))?;
        Ok(Response::default())
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Ownable(query_msg) => {
                return self
                    .ownable
                    .query(&deps, env, query_msg)
                    .map(|res| to_binary(&res))
                    .unwrap();
            }
            QueryMsg::Metadata(query_msg) => {
                return self
                    .metadata
                    .query(&deps, env, query_msg)
                    .map(|res| to_binary(&res))
                    .unwrap();
            }
        }
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const CONTRACT_INFO: Item<ContractVersion> = Item::new("contract_info");
