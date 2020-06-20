mod logger;
mod data;
mod input;
mod collector;
mod processor;
mod inference;
mod output;
mod config;
use config::Config;

use tokio::sync::oneshot;
use tokio::sync::mpsc::channel;
use signal_hook::{iterator::Signals, SIGINT};

#[tokio::main]
async fn main() {

    println!("Hello, banshee");

    // instantiate config
    let cfg_inst = Config::new();

    // init logger
    logger::init(cfg_inst.clone());
    // now logging is available

    let subsystems = async move {
        // channel to control input is a oneshot
        let (mut tx_stop, rx_stop) = oneshot::channel();
        
        // other channels are universal        
        // channel to pass fragments: input --> collector
        let (mut tx_frag, rx_frag) = channel::<data::Fragment>(100);
        // channel to pass sessions: collector --> processor
        let (mut tx_sess, rx_sess) = channel::<data::Session>(100);
        // channel to pass samples: procesor --> inference
        let (mut tx_smpl, rx_smpl) = channel::<data::FinalSample>(100);
        // channel to pass stored results: inference --> output
        let (mut tx_rslt, rx_rslt) = channel::<data::StoredResult>(100);

        // launch worker submodules
        input::run(cfg_inst.clone(), rx_stop, tx_frag.clone()).await;
        collector::run(cfg_inst.clone(), rx_frag, tx_sess.clone()).await;
        processor::run(cfg_inst.clone(), rx_sess, tx_smpl.clone()).await;
        inference::run(cfg_inst.clone(), rx_smpl, tx_rslt.clone()).await;
        output::run(cfg_inst.clone(), rx_rslt).await;

        // launch ctrl-c handler
        let signals = Signals::new(&[SIGINT]).unwrap();
        tokio::spawn(async move {
            for _ in signals.forever() {
                println!("Trying to stop banshee!");
                // send stop signal to all channels
                let _ = tx_stop.send(());                               // stops input
                let _ = tx_frag.send(data::Fragment::Stop).await;       // stops collector
                let _ = tx_sess.send(data::Session::Stop).await;        // stops processor
                let _ = tx_smpl.send(data::FinalSample::Stop).await;    // stops inference
                let _ = tx_rslt.send(data::StoredResult::Stop).await;   // stops output
                break;
            }
        }).await.unwrap();
    };

    println!("Banshee has started, press Ctrl+C to stop");
    subsystems.await;
}

