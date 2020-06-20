use crate::config::SharedConfig;
use crate::StopFlag;

use log::info;

use std::sync::atomic::Ordering;
use std::thread;
use std::time;

// timeout to wait processed session from processor
const TIMEOUT_WAIT_PROCESSED_SEC: u64 = 1;

pub async fn run(_cfg: SharedConfig, stop_flag: StopFlag) {
    info!("start inference");

    tokio::spawn(async move {

        loop {
            if stop_flag.load(Ordering::SeqCst) {
                info!("stop inference");
                break;
            }
    
            // imitate trying to receive processed sessions them:
            thread::sleep(time::Duration::from_secs(TIMEOUT_WAIT_PROCESSED_SEC));
    
            info!("inference: receiving processed data is not implemented yet");    
        }
    
    });
}
