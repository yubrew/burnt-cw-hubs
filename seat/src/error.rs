use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Ownable Module")]
    OwnableError(ownable::OwnableError),

    #[error("Metadata Module")]
    MetadataError(metadata::MetadataError),

    #[error("Token Module")]
    SeatTokenError(cw721_base::ContractError),

    #[error("Redeemable Module")]
    RedeemableError(redeemable::errors::ContractError),

    #[error("Metadata Module")]
    SellableError(sellable::errors::ContractError),

    #[error("Sales Module")]
    SalesError(sales::errors::ContractError),
}
