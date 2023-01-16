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

    #[error("Beneficiary must be different from the protected contract")]
    BeneficiaryMustBeDifferentFromProtectedContract {},

    #[error("Commission bps must be smaller than or equal to 10,000")]
    InvalidCommissionBps {},

    #[error("Nothing to upgrade")]
    NothingToUpdate,

    #[error("New contract owner must be different from the current contract owner")]
    NewContractOwnerMustBeDifferent,

    #[error("Not subscribed")]
    NotSubscribed {},

    #[error("Invalid Cw20HookMsg")]
    InvalidCw20HookMsg {},

    #[error("Error parsing instantiate reply")]
    ErrorParsingInstantiateReply {},

    #[error("Invalid reply id: {id:?}")]
    UnknownReplyId { id: u64 },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    //
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
