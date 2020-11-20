//! Tx-pool shared type define.
use crate::core::{error::OutPointError, Capacity, Cycle, FeeRate};
use crate::packed::Byte32;
use ckb_error::{Error, ErrorKind};
use failure::Fail;
use std::collections::HashMap;

/// TX reject message
#[derive(Debug, Fail)]
pub enum Reject {
    /// Transaction fee lower than config
    #[fail(
        display = "The min fee rate is {} shannons/KB, so the transaction fee should be {} shannons at least, but only got {}",
        _0, _1, _2
    )]
    LowFeeRate(FeeRate, u64, u64),

    /// Transaction exceeded maximum ancestors count limit
    #[fail(display = "Transaction exceeded maximum ancestors count limit, try send it later")]
    ExceededMaximumAncestorsCount,

    /// Transaction pool exceeded maximum size or cycles limit,
    #[fail(
        display = "Transaction pool exceeded maximum {} limit({}), try send it later",
        _0, _1
    )]
    Full(String, u64),

    /// Transaction already exist in transaction_pool
    #[fail(display = "Transaction({}) already exist in transaction_pool", _0)]
    Duplicated(Byte32),

    /// Malformed transaction
    #[fail(display = "Malformed {} transaction", _0)]
    Malformed(String),

    /// Resolve failed
    #[fail(display = "Resolve failed {}", _0)]
    Resolve(OutPointError),

    /// Verification failed
    #[fail(display = "Verification failed {}", _0)]
    Verification(Error),
}

impl From<Reject> for Error {
    fn from(error: Reject) -> Self {
        error.context(ErrorKind::SubmitTransaction).into()
    }
}

/// Tx-pool transaction status
#[derive(Debug, PartialEq, Eq)]
pub enum TxStatus {
    /// Status "pending". The transaction is in the pool, and not proposed yet.
    Pending,
    /// Status "proposed". The transaction is in the pool and has been proposed.
    Proposed,
}

/// Tx-pool entry info
#[derive(Debug, PartialEq, Eq)]
pub struct TxEntryInfo {
    /// Consumed cycles.
    pub cycles: Cycle,
    /// The transaction serialized size in block.
    pub size: u64,
    /// The transaction fee.
    pub fee: Capacity,
    /// Size of in-tx-pool ancestor transactions
    pub ancestors_size: u64,
    /// Cycles of in-tx-pool ancestor transactions
    pub ancestors_cycles: u64,
    /// Number of in-tx-pool ancestor transactions
    pub ancestors_count: u64,
}

/// Array of transaction ids
#[derive(Debug, PartialEq, Eq)]
pub struct TxPoolIds {
    /// Pending transaction ids
    pub pending: Vec<Byte32>,
    /// Proposed transaction ids
    pub proposed: Vec<Byte32>,
}

/// All in-pool transaction entry info
#[derive(Debug, PartialEq, Eq)]
pub struct TxPoolEntryInfo {
    /// Pending transaction entry info
    pub pending: HashMap<Byte32, TxEntryInfo>,
    /// Proposed transaction entry info
    pub proposed: HashMap<Byte32, TxEntryInfo>,
}
