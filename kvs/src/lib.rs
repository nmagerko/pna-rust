#![deny(missing_docs)]

//! `kvs` is a simple in-memory key-value store.
//!
//! # About
//!
//! This key value store is an implementation of the Rust practical applications project for the
//! [PingCAP talent plan](https://github.com/pingcap/talent-plan).

pub use kv::KvStore;

mod kv;
