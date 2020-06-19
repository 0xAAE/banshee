use clap::{Arg, App, ArgMatches};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::convert::Into;

mod core;
mod endpoint;
use self::core::ConfigCore;
use endpoint::Endpoint;

pub struct Config<'t> {
    args: ArgMatches<'t>,
    core: RwLock<ConfigCore>
}

pub type SharedConfig<'t> = Arc<Config<'t>>;

impl <'t> Config<'t> {

    pub fn new() -> SharedConfig<'t> {
        let args = init_args();
        let pathname = args.value_of("config").unwrap_or("banshee.ini");
        // init from file
        let inst = ConfigCore::new(pathname);
        // override values by args
        // todo ...
        // ready
        Arc::<Config>::new(Config {
            args: args,
            core: RwLock::new(inst)
        })
    }

    pub fn peers(&self) -> Vec<Endpoint> {
        let c = self.core.read().unwrap();
        c.peers().clone()
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
