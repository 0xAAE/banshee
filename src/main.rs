use tokio::prelude::*;

mod config;
use config::Config;

#[tokio::main]
async fn main() {
    // config
    let cfg_inst = Config::new();
    // instantiate
    let peers = cfg_inst.peers();
    for p in peers {
        println!("Connect to {}", p);
    }
    // run
}
