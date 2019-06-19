#![deny(missing_docs)]

//! `kvs` is a simple key-value store backed by a log of operations persisted to disk. The log
//! stores JSON objects, meaning that it is human readable but should not be edited manually.
//!
//! The log file is compacted whenever its size exceeds `COMPACT_BYTES` bytes to avoid unbounded
//! disk usage.
//!
//! # About
//!
//! This key value store is an implementation of the Rust practical applications project for the
//! [PingCAP talent plan](https://github.com/pingcap/talent-plan).

#[macro_use]
extern crate failure_derive;

pub use error::{KvError, Result};
pub use kv::KvStore;

mod error;
mod kv;
