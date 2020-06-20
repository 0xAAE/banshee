use crate::config::SharedConfig;
use crate::StopFlag;

use log::info;

use std::sync::atomic::Ordering;
use std::thread;
use std::time;

// timeout to wait result from inference to send to storage
const TIMEOUT_WAIT_RESULT_SEC: u64 = 1;

pub async fn run(_cfg: SharedConfig, stop_flag: StopFlag) {
    info!("start output");

    tokio::spawn(async move {
        loop {
            if stop_flag.load(Ordering::SeqCst) {
                info!("stop output");
                break;
            }
    
            // imitate trying to receive fragments:
            thread::sleep(time::Duration::from_secs(TIMEOUT_WAIT_RESULT_SEC));
    
            info!("output: receiving inferred data is not implemented yet");
        }     
    });
}
