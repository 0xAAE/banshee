use clap::{Arg, App, ArgMatches};
use log::LevelFilter;
use std::sync::{Arc, RwLock};

mod core;
mod endpoint;
use self::core::ConfigCore;

pub struct Config {
    core: RwLock<ConfigCore>
}

pub type SharedConfig = Arc<Config>;
pub type Endpoint = endpoint::Endpoint;

impl Config{

    pub fn new() -> SharedConfig {
        let args = init_args();
        let pathname = args.value_of("config").unwrap_or("banshee.ini");
        // init from file
        let inst = ConfigCore::new(pathname);
        // override values by args
        // todo ...
        // ready
        Arc::<Config>::new(Config {
            core: RwLock::new(inst)
        })
    }

    pub fn peers(&self) -> Vec<Endpoint> {
        let c = self.core.read().unwrap();
        c.peers().clone()
    }

    pub fn log_lvl_console(&self) -> LevelFilter {
        // todo: obtain value from config file
        LevelFilter::Info
    }

    pub fn log_lvl_file(&self) -> LevelFilter {
        // todo: obtain value from config file
        LevelFilter::Debug
    }
}

// command line
fn init_args() -> ArgMatches<'static> {
    App::new("banshee")
        .version("0.1")
        .about("sound stream processor")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .default_value("banshee.ini")
            .help("pathname to configuration file")
            .takes_value(true))
        .get_matches()
}
