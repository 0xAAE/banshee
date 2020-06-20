use crate::config::SharedConfig;
use crate::StopFlag;

use log::info;

use std::sync::atomic::Ordering;
use std::thread;
use std::time;

// timeout to wait session from collector
const TIMEOUT_WAIT_SESSION_SEC: u64 = 2;

pub async fn run(_cfg: SharedConfig, stop_flag: StopFlag) {
    info!("start processor");

    tokio::spawn(async move {

        loop {
            if stop_flag.load(Ordering::SeqCst) {
                info!("stop processor");
                break;
            }
    
            // imitate trying to receive fragments:
            thread::sleep(time::Duration::from_secs(TIMEOUT_WAIT_SESSION_SEC));
    
            info!("processor: receiving sessions is not implemented yet");    
        }
    
    });
}
