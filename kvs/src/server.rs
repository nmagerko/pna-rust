extern crate bincode;

use crate::{KvRequest, KvResponse, KvsEngine, Result};
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

/// A server for hosting a key value store
pub struct KvServer<E: KvsEngine> {
    addr: SocketAddr,
    engine: E,
}

impl<E: KvsEngine> KvServer<E> {
    /// Creates a new server ready to accept key-value requests
    ///
    /// # Arguments
    ///
    /// - addr - the address to bind to
    /// - engine - the engine to use for storage
    pub fn new(addr: SocketAddr, engine: E) -> KvServer<E> {
        KvServer { addr, engine }
    }

    /// Waits for incoming connections indefinitely (until killed)
    ///
    /// # Errors
    ///
    /// An error may occur if there is a problem binding to the bind address
    pub fn serve(&mut self) -> Result<()> {
        let listener = TcpListener::bind(self.addr)?;
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    info!("New connection from {}", stream.peer_addr().unwrap().ip());
                    self.handle_request(&mut stream);
                    match stream.shutdown(Shutdown::Write) {
                        Ok(_) => {}
                        Err(err) => {
                            warn!("Failed to close socket: {}", err);
                        }
                    }
                }
                Err(err) => {
                    warn!("Failed while accepting stream: {}", err);
                }
            };
        }
        Ok(())
    }

    fn handle_request(&mut self, stream: &mut TcpStream) {
        let mut request_buf = Vec::new();
        if let Err(err) = stream.read_to_end(&mut request_buf) {
            error!("Failed while reading request: {}", err);
            return;
        };

        let request = match bincode::deserialize(&request_buf) {
            Ok(cmd) => cmd,
            Err(err) => {
                error!("Failed while deserializing request: {}", err);
                return;
            }
        };
        let response = match request {
            KvRequest::Get { key } => match self.engine.get(key) {
                Ok(value) => KvResponse::Get { value },
                Err(err) => KvResponse::Error {
                    message: err.to_string(),
                },
            },
            KvRequest::Remove { key } => match self.engine.remove(key) {
                Ok(_) => KvResponse::Remove {},
                Err(err) => KvResponse::Error {
                    message: err.to_string(),
                },
            },
            KvRequest::Set { key, value } => match self.engine.set(key, value) {
                Ok(_) => KvResponse::Set {},
                Err(err) => KvResponse::Error {
                    message: err.to_string(),
                },
            },
        };

        let serialized = match bincode::serialize(&response) {
            Ok(value) => value,
            Err(err) => {
                error!("Failed while serializing request: {}", err);
                return;
            }
        };
        if let Err(err) = stream.write_all(&serialized) {
            error!("Failed while writing response: {}", err);
        }
    }
}
