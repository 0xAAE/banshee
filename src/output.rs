use crate::config::SharedConfig;
use crate::data::StoredResult;

use log::{error, info};
use tokio::sync::mpsc::{Sender, Receiver};

pub async fn run(_cfg: SharedConfig, mut rx_rslt: Receiver<StoredResult>) {
    info!("start output");

    tokio::spawn(async move {

        loop {
            match rx_rslt.recv().await {
                None => {
                    error!("stored result input channel is broken");
                    break;
                },
                Some(f) => {
                    match f {
                        StoredResult::Stop => {
                            info!("stop output");
                            break;
                        },
                        _ => {
                            //todo: handle stored result
                            info!("output: handling results is not implemented yet");
                        }
                    }                    
                }
            }
        }    
        info!("output is stopped");

    });
}
