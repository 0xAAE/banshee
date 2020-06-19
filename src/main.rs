use tokio::prelude::*;

mod config;
use config::Config;

#[tokio::main]
async fn main() {
    // get config
    let cfg_inst = Config::new();

    // instantiate, now simply print target peers to connect to
    let peers = cfg_inst.peers();
    for p in peers {
        println!("Connect to {}", p);
    }
    // run
}
