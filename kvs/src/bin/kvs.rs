extern crate structopt;

use kvs::KvStore;
use std::process;
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();
    let mut store = KvStore::new();

    match opts.cmd {
        Command::Get { key } => {
            let _value = store.get(key);
            print_unimplemented();
        }
        Command::Set { key, value } => {
            store.set(key, value);
            print_unimplemented();
        }
        Command::Remove { key } => {
            store.remove(key);
            print_unimplemented();
        }
    }
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

fn print_unimplemented() {
    eprintln!("unimplemented");
    process::exit(1);
}
