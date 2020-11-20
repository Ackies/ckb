//! The error type for Tx-pool operations

pub use ckb_types::core::tx_pool::Reject;
use failure::Fail;
use tokio::sync::mpsc::error::TrySendError as TokioTrySendError;

/// The error type for block assemble related
#[derive(Debug, PartialEq, Clone, Eq, Fail)]
pub enum BlockAssemblerError {
    /// Input is invalid
    #[fail(display = "InvalidInput")]
    InvalidInput,
    /// Parameters is invalid
    #[fail(display = "InvalidParams {}", _0)]
    InvalidParams(String),
    /// BlockAssembler is disabled
    #[fail(display = "Disabled")]
    Disabled,
}

#[derive(Fail, Debug)]
#[fail(display = "TrySendError {}.", _0)]
pub(crate) struct TrySendError(String);

pub(crate) fn handle_try_send_error<T>(error: TokioTrySendError<T>) -> (T, TrySendError) {
    let e = TrySendError(format!("{}", error));
    let m = match error {
        TokioTrySendError::Full(t) | TokioTrySendError::Closed(t) => t,
    };
    (m, e)
}
