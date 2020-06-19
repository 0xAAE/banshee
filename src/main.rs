use tokio::prelude::*;
use log::{info, debug};

mod config;
mod logger;
use config::Config;

#[tokio::main]
async fn main() {
    // get config
    let cfg_inst = Config::new();
    // init logger
    logger::init(cfg_inst.clone());
    // instantiate, now simply print target peers to connect to
    debug!("Hello, banshee");
    let peers = cfg_inst.peers();
    for p in peers {
        info!("Connect to {}", p);
        // todo: start tokio client
    }
    // run
    debug!("running");
    debug!("exit");
}
