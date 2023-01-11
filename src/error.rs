use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Query entry point is unsupported")]
    QueryUnsupported,

    #[error("Denom `{denom}` does not represent a wrapped token: {reason}")]
    NotWrappedToken {
        denom: String,
        reason: String,
    }
}
