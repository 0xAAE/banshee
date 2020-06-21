//! Получение входных данных.
//! 
//! Задачи:
//! *  подключиться к пулу подсистем сопряжения по TCP
//! *  выполнять двусторонний обмен по логике взаимодействия между подсистемами по V-протоколу
//! *  читать фрагменты объектов по V-протоколу
//! *  по полю в заголовке фрагмента определить получателя фрагмента
//! *  передать фрагмент в коллектор

use crate::config::SharedConfig;
use crate::data::Fragment;

use log::info;
use tokio::sync::oneshot;
use tokio::sync::mpsc::Sender;

/// Запускает в асинхронном режиме подсистему получения входных данных от системы сопряжения комплекса
/// 
/// Параметры:
/// 
/// * `cfg` - доступ к общей разделяемой конфигурации приложения. Потокобезопасность обеспечивается самим объектом
/// * `rx_stop` - межпоточный канал получения команды на завершение, читатель
/// * `tx_frag` - межпоточный канал передачи в коллектор получаемых фрагментов, писатель
/// 
/// Возвращает Future для ожидания в вызывающем коде
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
