use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Ownable Module Error")]
    OwnableError(ownable::OwnableError),

    #[error("Metadata Module Error")]
    MetadataError(metadata::MetadataError),

    #[error("Token Module Error")]
    SeatTokenError(cw721_base::ContractError),

    #[error("Redeemable Module Error")]
    RedeemableError(redeemable::errors::ContractError),

    #[error("Metadata Module Error")]
    SellableError(sellable::errors::ContractError),

    #[error("Sales Module Error")]
    SalesError(sales::errors::ContractError),
}
