use crate::config::SharedConfig;
use crate::data::{Fragment, Session};

use log::{info, error};
use tokio::sync::mpsc::{Sender, Receiver};

pub async fn run(_cfg: SharedConfig, mut rx_frag: Receiver<Fragment>, _tx_sess: Sender<Session>) {
    info!("start collector");

    tokio::spawn(async move {

        loop {
            match rx_frag.recv().await {
                None => {
                    error!("fragments input channel is broken");
                    break;
                },
                Some(f) => {
                    match f {
                        Fragment::Stop => {
                            info!("stop collector");
                            break;
                        },
                        _ => {
                            //todo: handle fragment
                            info!("collector: handling fragments is not implemented yet");
                        }
                    }                    
                }
            }
        }    
        info!("collector is stopped");

    });
}
