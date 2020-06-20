use crate::config::SharedConfig;
use crate::StopFlag;

use log::info;

use std::sync::atomic::Ordering;
use std::thread;
use std::time;

// pause in seconds between attempts to connect to remote peer
const TIMEOUT_CONNECT_PEER_SEC: u64 = 5;

pub async fn run(cfg: SharedConfig, stop_flag: StopFlag) {
    info!("start input");

    tokio::spawn( async move {

        loop {
            if stop_flag.load(Ordering::SeqCst) {
                info!("stop input");
                break;
            }
    
            // imitate trying to connect:
            thread::sleep(time::Duration::from_secs(TIMEOUT_CONNECT_PEER_SEC));
    
            // print target peers to connect to
            let peers = cfg.peers();
            for p in peers {
                info!("input: unable to connect to {}", p);
            }
        }

    });
}
