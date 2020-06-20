mod logger;
mod input;
mod collector;
mod processor;
mod inference;
mod output;
mod config;
use config::Config;

use log::info;
use signal_hook::{iterator::Signals, SIGINT};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time;

type StopFlag = Arc<AtomicBool>;

#[tokio::main]
async fn main() {

    println!("Hello, banshee");

    // instantiate config
    let cfg_inst = Config::new();

    // init logger
    logger::init(cfg_inst.clone());
    // now logging is available

    let stop_flag: StopFlag = Arc::new(AtomicBool::new(false));

    let subsystems = async move {
        input::run(cfg_inst.clone(), stop_flag.clone()).await;
        collector::run(cfg_inst.clone(), stop_flag.clone()).await;
        processor::run(cfg_inst.clone(), stop_flag.clone()).await;
        inference::run(cfg_inst.clone(), stop_flag.clone()).await;
        output::run(cfg_inst.clone(), stop_flag.clone()).await;

        let signals = Signals::new(&[SIGINT]).unwrap();
        tokio::spawn(async move {
            for sig in signals.forever() {
                println!("Received signal {:?}", sig);
                stop_flag.store(true, Ordering::SeqCst);
                println!("Trying to stop banshee!");
                break;
            }
        }).await.unwrap();
    };

    subsystems.await;

    println!("Banshee has started");
}
