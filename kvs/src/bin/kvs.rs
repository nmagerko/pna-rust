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
        Some("key") => print_unimplemented(),
        Some("set") => print_unimplemented(),
        Some("rm") => print_unimplemented(),
        _ => {
            eprintln!("No such subcommand");
            process::exit(1);
        }
    }
}

fn print_unimplemented() {
    eprintln!("unimplemented");
    process::exit(1);
}
