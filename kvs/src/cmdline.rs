extern crate structopt;

use crate::{KvError, Result};

/// Parses an address formatted as a string to a host and port
///
/// # Arguments
///
/// `addr` - a string slice formatted as 'host:port'
///
/// # Errors
///
/// - `KvError::BadAddressError` when the address is malformated
pub fn parse_addr(addr: &str) -> Result<(String, u32)> {
    let split: Vec<_> = addr.split(':').collect();
    if split.len() != 2 {
        return Err(KvError::BadAddressError("Missing ':'".to_owned()));
    }
    let host = split[0].to_owned();
    let port = match split[1].parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            return Err(KvError::BadAddressError("Invalid port number".to_owned()));
        }
    };
    Ok((host, port))
}

/// Parses the verbosity flag, defaulting to INFO-level verbosity
/// when the flag is not present
///
/// # Arguments
///
/// `flag` - the verbosity flag (if present)
pub fn parse_verbosity(flag: &str) -> Result<usize> {
    return Ok(flag.matches("v").count() + 1);
}
