//! Выполняет обработку полученных объектов при помощи подключаемых модулей.
//! 
//! Обеспечивает последовательную обработку каждого объекта заданным набором фильтров.
//! Обеспечивает параллельную обработку разных объектов. Задачи по каждому объекту:
//! *  получить готовый объект от коллектора
//! *  обработать объект цепочкой фильтров
//! *  передать полученный результат в подмодуль расчета итогового результата (inference)

use crate::config::SharedConfig;
use crate::data::{Session, FinalSample};

use log::{info, error};
use tokio::sync::mpsc::{Sender, Receiver};

/// Запускает в асинхронном режиме подсистему обработки полученных от коллектора сессий
/// 
/// Параметры:
/// 
/// * `cfg` - доступ к общей разделяемой конфигурации приложения. Потокобезопасность обеспечивается самим объектом
/// * `rx_sess` - межпоточный канал получения готовых к обработке сессий, читатель
/// * `tx_smpl` - межпоточный канал передачи обработанных сэмплов в модуль inference, писатель
/// 
/// Возвращает Future для ожидания в вызывающем коде
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
