#![deny(missing_docs)]

//! `kvs` is a simple key-value store that uses an in-memory index which is persisted to disk.
//!
//! # About
//!
//! This key value store is an implementation of the Rust practical applications project for the
//! [PingCAP talent plan](https://github.com/pingcap/talent-plan).

#[macro_use] extern crate failure_derive;

pub use error::Result;
pub use kv::KvStore;

mod error;
mod kv;
