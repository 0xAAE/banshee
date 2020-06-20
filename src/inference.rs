use crate::config::SharedConfig;
use crate::data::{FinalSample, StoredResult};

use log::{info, error};
use tokio::sync::mpsc::{Sender, Receiver};

pub async fn run(_cfg: SharedConfig, mut rx_smpl: Receiver<FinalSample>, _tx_rslt: Sender<StoredResult>) {
    info!("start inference");

    tokio::spawn(async move {

        loop {
            match rx_smpl.recv().await {
                None => {
                    error!("final samples input channel is broken");
                    break;
                },
                Some(f) => {
                    match f {
                        FinalSample::Stop => {
                            info!("stop inference");
                            break;
                        },
                        _ => {
                            //todo: handle sample
                            info!("inference: handling samples is not implemented yet");
                        }
                    }                    
                }
            }
        }    
        info!("inference is stopped");

    });
}
