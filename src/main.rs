use tokio::prelude::*;
use log::{info, debug};

mod config;
mod logger;
use config::Config;

#[tokio::main]
async fn main() {

    // instantiate config
    let cfg_inst = Config::new();

    // init logger
    logger::init(cfg_inst.clone());
    // now can log
    info!("Hello, banshee");

    // print target peers to connect to
    let peers = cfg_inst.peers();
    for p in peers {
        info!("Connect to {}", p);
        // todo: start tokio client
    }
 
    // run input
    info!("start input");

    // run collector
    info!("start collector");

    // run processor
    info!("start processor");

    // run inference
    info!("start inference");

    // run output
    info!("start otput");

    info!("exit");
}
