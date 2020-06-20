use crate::config::SharedConfig;
use crate::StopFlag;

use log::info;

use std::sync::atomic::Ordering;
use std::thread;
use std::time;

// timeout to wait fragments from input
const TIMEOUT_WAIT_FRAGMENT_SEC: u64 = 1;

pub async fn run(_cfg: SharedConfig, stop_flag: StopFlag) {
    info!("start collector");

    tokio::spawn(async move {

        loop {
            if stop_flag.load(Ordering::SeqCst) {
                info!("stop collector");
                break;
            }
    
            // imitate trying to receive fragments from input:
            thread::sleep(time::Duration::from_secs(TIMEOUT_WAIT_FRAGMENT_SEC));
    
            info!("collector: receiving fragments is not implemented yet");
        }    

    });
}
