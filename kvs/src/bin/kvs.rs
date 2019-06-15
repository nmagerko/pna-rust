extern crate clap;
use clap::{App, Arg, SubCommand};
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

    match matches.subcommand_name() {
        Some("key") => {},
        Some("set") => {},
        Some("rm") => {},
        _ => {
            eprintln!("No such subcommand");
            process::exit(1);
        }
    }
}
