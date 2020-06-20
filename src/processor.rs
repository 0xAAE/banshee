use crate::config::SharedConfig;
use crate::data::{Session, FinalSample};

use log::{info, error};
use tokio::sync::mpsc::{Sender, Receiver};

pub async fn run(_cfg: SharedConfig, mut rx_sess: Receiver<Session>, _tx_smpl: Sender<FinalSample>) {
    info!("start processor");

    tokio::spawn(async move {

        loop {
            match rx_sess.recv().await {
                None => {
                    error!("sessions input channel is broken");
                    break;
                },
                Some(f) => {
                    match f {
                        Session::Stop => {
                            info!("stop processor");
                            break;
                        },
                        _ => {
                            //todo: handle session
                            info!("processor: handling sessions is not implemented yet");
                        }
                    }                    
                }
            }
        }    
        info!("processor is stopped");
    
    });
}
