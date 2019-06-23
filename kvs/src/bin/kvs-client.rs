extern crate kvs;

extern crate stderrlog;
extern crate structopt;

use kvs::{KvClient, KvRequest, Result};
use std::net::SocketAddr;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opts::from_args();
    stderrlog::new()
        .quiet(opts.quiet)
        .verbosity(2)
        .init()
        .unwrap();

    let client = KvClient::new(opts.addr);
    match opts.cmd {
        Command::Get { key } => client.send(KvRequest::Get { key }),
        Command::Set { key, value } => client.send(KvRequest::Set { key, value }),
        Command::Remove { key } => client.send(KvRequest::Remove { key }),
    }?;
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-client")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(long = "addr", default_value = r#"127.0.0.1:4000"#)]
    addr: SocketAddr,
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Remove { key: String },
}
