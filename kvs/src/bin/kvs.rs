extern crate clap;
use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process;

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!(", "))
        .about("A key-value store written in Rust")
        .subcommand(SubCommand::with_name("get")
            .arg(Arg::with_name("key").required(true)))
        .subcommand(SubCommand::with_name("set")
            .arg(Arg::with_name("key").required(true))
            .arg(Arg::with_name("value").required(true)))
        .subcommand(SubCommand::with_name("rm")
            .arg(Arg::with_name("key").required(true)))
        .get_matches();

    let mut store = KvStore::new();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("key").unwrap().to_string();
            let _value = store.get(key);
            print_unimplemented();
        },
        ("set", Some(matches)) => {
            let key = matches.value_of("key").unwrap().to_string();
            let value = matches.value_of("value").unwrap().to_string();
            store.set(key, value);
            print_unimplemented();
        },
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").unwrap().to_string();
            store.remove(key);
            print_unimplemented();
        }
        _ => {
            eprintln!("Invalid usage: no such subcommand");
            process::exit(1);
        }
    }
}

fn print_unimplemented() {
    eprintln!("unimplemented");
    process::exit(1);
}
