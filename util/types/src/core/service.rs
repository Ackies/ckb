//! Types for CKB services.
//!
//! A CKB service acts as an actor, which processes requests from a channel and sends back the
//! response via one shot channel.
use crate::core::{tx_pool::Reject, Capacity, Cycle, TransactionView};
use ckb_channel::Sender;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

/// Default channel size to send control signals.
pub const SIGNAL_CHANNEL_SIZE: usize = 1;
/// Default channel size to send messages.
pub const DEFAULT_CHANNEL_SIZE: usize = 32;

/// Synchronous request sent to the service.
pub struct Request<A, R> {
    /// One shot channel for the service to send back the response.
    pub responder: mpsc::Sender<R>,
    /// Request arguments.
    pub arguments: A,
}

impl<A, R> Request<A, R> {
    /// Call the service with the arguments and wait for the response.
    pub fn call(sender: &Sender<Request<A, R>>, arguments: A) -> Option<R> {
        let (responder, response) = mpsc::channel();
        let _ = sender.send(Request {
            responder,
            arguments,
        });
        response.recv().ok()
    }
}

/// Notify pool transaction entry
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PoolTransactionEntry {
    /// Transaction view
    pub transaction: TransactionView,
    /// Transaction consumed cycles
    pub cycles: Cycle,
    /// Transaction serialized cycles
    pub size: usize,
    /// Transaction fee
    pub fee: Capacity,
}

/// TX reject message
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoolTransactionReject {
    /// Transaction fee lower than config
    LowFeeRate(String),

    /// Transaction exceeded maximum ancestors count limit
    ExceededMaximumAncestorsCount(String),

    /// Transaction pool exceeded maximum size or cycles limit,
    Full(String),

    /// Transaction already exist in transaction_pool
    Duplicated(String),

    /// Malformed transaction
    Malformed(String),

    /// Resolve failed
    Resolve(String),

    /// Verification failed
    Verification(String),
}

impl From<&Reject> for PoolTransactionReject {
    fn from(reject: &Reject) -> Self {
        match reject {
            Reject::LowFeeRate(..) => Self::LowFeeRate(format!("{}", reject)),
            Reject::ExceededMaximumAncestorsCount => {
                Self::ExceededMaximumAncestorsCount(format!("{}", reject))
            }
            Reject::Full(..) => Self::Full(format!("{}", reject)),
            Reject::Duplicated(_) => Self::Duplicated(format!("{}", reject)),
            Reject::Malformed(_) => Self::Malformed(format!("{}", reject)),
            Reject::Resolve(_) => Self::Resolve(format!("{}", reject)),
            Reject::Verification(_) => Self::Verification(format!("{}", reject)),
        }
    }
}
