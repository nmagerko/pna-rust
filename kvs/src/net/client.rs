extern crate bincode;

use crate::{KvsRequest, KvsResponse, Result};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpStream};

/// A client for interacting with a remote key-value store
pub struct KvsClient {
    addr: SocketAddr,
}

impl KvsClient {
    /// Creates a new client ready to connect at the given address
    ///
    /// # Arguments
    ///
    /// - addr - the address of the server
    pub fn new(addr: SocketAddr) -> KvsClient {
        KvsClient { addr }
    }

    /// Sends a request to the server at the stored address
    ///
    /// # Arguments
    ///
    /// - request - a request to send to the server
    ///
    /// # Errors
    ///
    /// An error may occur due to a failure to connect to the server,
    /// problems with serialization/deserialization, or other networking errors
    pub fn send(&self, request: KvsRequest) -> Result<KvsResponse> {
        let mut stream = TcpStream::connect(self.addr)?;

        let serialized = bincode::serialize(&request).expect("Failed to serialize request");
        stream.write_all(&serialized)?;
        stream.shutdown(Shutdown::Write)?;

        let mut read_buf = Vec::new();
        stream.read_to_end(&mut read_buf)?;

        Ok(bincode::deserialize::<KvsResponse>(&read_buf).expect("Failed to deserialize request"))
    }
}
