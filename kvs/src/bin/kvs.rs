extern crate structopt;

use kvs::{KvError, KvStore, Result};
use std::process;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opts::from_args();
    let mut store = KvStore::new()?;

    match opts.cmd {
        Command::Get { key } => {
            match store.get(key) {
                Ok(Some(value)) => println!("{}", value),
                Ok(None) => {
                    println!("Key not found");
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            };
        }
        Command::Set { key, value } => {
            store.set(key, value)?;
        }
        Command::Remove { key } => {
            match store.remove(key) {
                Ok(()) => {}
                Err(KvError::BadRemovalError(_)) => {
                    println!("Key not found");
                    process::exit(1);
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            };
        }
    };

    Ok(())
}

#[derive(StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Remove { key: String },
}
