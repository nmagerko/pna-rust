extern crate kvs;

extern crate stderrlog;
extern crate structopt;

use kvs::{KvsClient, KvsRequest, KvsResponse, Result};
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opts::from_args();
    stderrlog::new()
        .quiet(opts.quiet)
        .verbosity(2)
        .init()
        .unwrap();

    let response = match opts.cmd {
        Command::Get { addr, key } => {
            let client = KvsClient::new(addr);
            client.send(KvsRequest::Get { key })
        }
        Command::Set { addr, key, value } => {
            let client = KvsClient::new(addr);
            client.send(KvsRequest::Set { key, value })
        }
        Command::Remove { addr, key } => {
            let client = KvsClient::new(addr);
            client.send(KvsRequest::Remove { key })
        }
    }?;
    match response {
        KvsResponse::Get { value } => match value {
            Some(value) => println!("{}", value),
            None => {
                println!("Key not found");
            }
        },
        KvsResponse::Error { message } => {
            eprintln!("{}", message);
            exit(1);
        }
        _ => {}
    };
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-client")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "get")]
    Get {
        #[structopt(long = "addr", default_value = r#"127.0.0.1:4000"#)]
        addr: SocketAddr,
        key: String,
    },
    #[structopt(name = "set")]
    Set {
        #[structopt(long = "addr", default_value = r#"127.0.0.1:4000"#)]
        addr: SocketAddr,
        key: String,
        value: String,
    },
    #[structopt(name = "rm")]
    Remove {
        #[structopt(long = "addr", default_value = r#"127.0.0.1:4000"#)]
        addr: SocketAddr,
        key: String,
    },
}
