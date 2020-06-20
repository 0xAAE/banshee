use crate::config::SharedConfig;
use crate::data::Fragment;

use log::info;
use tokio::sync::oneshot;
use tokio::sync::mpsc::Sender;

pub async fn run(cfg: SharedConfig, rx_stop: oneshot::Receiver<()>, _tx_frag: Sender<Fragment>) {
    info!("start input");

    tokio::spawn( async move {

        //todo: start all client sockets according to cfg.peers()
        // print target peers to connect to
        let peers = cfg.peers();
        for p in peers {
            info!("input: unable to connect to {}", p);
        }

        loop {
            let _ = rx_stop.await;
            info!("stop input");
            //todo: stop all client sockets
            break;
        }    
        info!("input is stopped");

    });
}
