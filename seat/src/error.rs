use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Ownable Module Error")]
    OwnableError(#[from] ownable::OwnableError),

    #[error("Metadata Module Error")]
    MetadataError(#[from] metadata::MetadataError),

    #[error("Token Module Error")]
    SeatTokenError(#[from] cw721_base::ContractError),

    #[error("Redeemable Module Error")]
    RedeemableError(#[from] redeemable::errors::ContractError),

    #[error(transparent)]
    SellableError(#[from] sellable::errors::ContractError),

    #[error("Sales Module Error")]
    SalesError(#[from] sales::errors::ContractError),
}
