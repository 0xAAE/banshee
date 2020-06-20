mod logger;
mod input;
mod collector;
mod processor;
mod inference;
mod output;
mod config;
use config::Config;

//use tokio::prelude::*;
use log::{info, debug};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time;

type StopFlag = Arc<AtomicBool>;

#[tokio::main]
async fn main() {

    // instantiate config
    let cfg_inst = Config::new();

    // init logger
    logger::init(cfg_inst.clone());
    // now logging is available
    info!("Hello, banshee");

    let stop_flag: StopFlag = Arc::new(AtomicBool::new(false));

    let submodules = async move {
        input::run(cfg_inst.clone(), stop_flag.clone()).await;
        collector::run(cfg_inst.clone(), stop_flag.clone()).await;
        processor::run(cfg_inst.clone(), stop_flag.clone()).await;
        inference::run(cfg_inst.clone(), stop_flag.clone()).await;
        output::run(cfg_inst.clone(), stop_flag.clone()).await;

        // imitate 5 sec work:
        thread::sleep(time::Duration::from_secs(15));
        stop_flag.store(true, Ordering::SeqCst);
    };

    submodules.await;

    info!("exit main");
}
