//! Выполняет обработку полученных объектов при помощи Tensor RT.
//! Получает итоговый результат для отправки в подсистему хранения.
//! Обеспечивает параллельную обработку полученных объектов на отдельных GPU.
//! Задачи по каждому объекту:
//! *  сформировать данные для TensorRT - массив [f16] нужного размера
//! *  запустить TensorRT на сформированных данных
//! *  получить результат
//! *  отправить результат в output

use crate::config::SharedConfig;
use crate::data::{FinalSample, StoredResult};

use log::{info, error};
use tokio::sync::mpsc::{Sender, Receiver};

/// Запускает в асинхронном режиме подсистему расчета итогового результата обработки звуковых сэмплов, полученных от процессора
/// 
/// Параметры:
/// 
/// * `cfg` - доступ к общей разделяемой конфигурации приложения. Потокобезопасность обеспечивается самим объектом
/// * rx_smpl` - межпоточный канал получения обработанных сэмплов, читатель
/// * tx_rslt` - межпоточный канал передачи результата в модуль отправки в систему сохранения, писатель
/// 
/// Возвращает Future для ожидания в вызывающем коде
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
