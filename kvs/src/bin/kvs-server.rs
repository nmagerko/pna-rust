extern crate kvs;

#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate structopt;

use kvs::{KvError, KvServer, KvStore, Result, SledKvsEngine};
use std::env;
use std::io::{Read, Write};
use std::net::SocketAddr;
use structopt::StructOpt;


const ENGINE_FILE: &str = ".engine";
const KVS_ENGINE: &str = "kvs";
const SLED_ENGINE: &str = "sled";

fn main() -> Result<()> {
    let opts = Opts::from_args();
    stderrlog::new()
        .quiet(opts.quiet)
        .verbosity(2)
        .init()
        .unwrap();

    info!("Version {}", env!("CARGO_PKG_VERSION"));
    info!("Bind address {}", opts.addr);
    info!("Engine {}", opts.engine_name);

    if !check_engine(&opts.engine_name)? {
        return Err(KvError::EngineMismatchError);
    }
    set_engine(&opts.engine_name)?;

    match &opts.engine_name[..] {
        KVS_ENGINE => {
            let engine = KvStore::new()?;
            KvServer::new(opts.addr, engine).serve()?;
        }
        SLED_ENGINE => {
            let engine = SledKvsEngine::new()?;
            KvServer::new(opts.addr, engine).serve()?;
        }
        _ => {
            panic!("Disallowed engine type found");
        }
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-server")]
struct Opts {
    #[structopt(long = "addr", default_value = r#"127.0.0.1:4000"#)]
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

fn check_engine(engine: &str) -> Result<bool> {
    let path = env::current_dir()?;
    let engine_file = std::path::Path::new(ENGINE_FILE);
    let engine_path = path.join(engine_file);
    if !engine_path.exists() {
        return Ok(true);
    }

    let mut content = Vec::new();
    std::fs::File::open(engine_path)?.read_to_end(&mut content)?;
    match std::str::from_utf8(&content) {
        Ok(identity) => Ok(identity == engine),
        Err(_) => Err(KvError::InternalError("UTF decode error".to_owned())),
    }
}

fn set_engine(engine: &str) -> Result<()> {
    let path = env::current_dir()?;
    let engine_file = std::path::Path::new(ENGINE_FILE);
    let engine_path = path.join(engine_file);
    std::fs::File::create(engine_path)?.write_all(engine.as_bytes())?;
    Ok(())
}
