use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

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
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
