extern crate kvs;

#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate structopt;

use kvs::Result;
use std::net::SocketAddr;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opts = Opts::from_args();
    stderrlog::new()
        .module(module_path!())
        .quiet(opts.quiet)
        .verbosity(2)
        .init()
        .unwrap();

    info!("Version {}", env!("CARGO_PKG_VERSION"));
    info!("Bind address {}", opts.addr);
    info!("Engine {}", opts.engine_name);
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-server")]
struct Opts {
    #[structopt(
        long = "addr",
        default_value = r#"127.0.0.1:4000"#,
        // parse(try_from_str = "parse_addr")
    )]
    addr: SocketAddr,
    #[structopt(
        long = "engine",
        default_value = r#"kvs"#,
        raw(possible_values = r#"&["kvs", "sled"]"#)
    )]
    engine_name: String,
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
}
