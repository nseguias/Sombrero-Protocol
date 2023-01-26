use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    ParseReplyError(#[from] ParseReplyError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Bounty % must be smaller than or equal to 100%")]
    InvalidBountyPercentage {},

    #[error("Nothing to upgrade")]
    NothingToUpdate,

    #[error("Invalid reply id: {id:?}")]
    UnknownReplyId { id: u64 },

    #[error("Protocol fee must be smaller than or equal to 100%")]
    InvalidProtocolFee {},

    #[error("Nothing to withdraw")]
    NothingToWithdraw {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    //
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
