#![deny(missing_docs)]

//! `kvs` is a key-value storage library that provides a filesystem-backed key-value store called
//! `KvStore`. The storage interface is generic and provides options for using other key-value
//! engines in place of the provided one, such as sled.
//!
//! Interaction with the key-value store is done remotely. `kvs-server` is a provided binary that
//! allows for hosting the store, while `kvs-client` is a binary that allows for get, set, and
//! removal requests to be sent to the hosted store using a custom binary protocol.
//!
//! # About
//!
//! This key value store is an implementation of the Rust practical applications project for the
//! [PingCAP talent plan](https://github.com/pingcap/talent-plan).

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

pub use client::KvClient;
pub use common::{KvRequest, KvResponse};
pub use engine::{KvStore, KvsEngine, SledKvsEngine};
pub use error::{KvError, Result};
pub use server::KvServer;

mod client;
mod common;
mod engine;
mod error;
mod server;
