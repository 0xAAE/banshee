//! Отправляет полученные от обработчика объектов результаты в подсистему хранения.
//! 
//! Задачи:
//! *  получить результат обработки очередного объекта
//! *  подготовить данные отправки
//! *  отправить данные в систему хранения
//! *  контролировать канал связи с системой хранения результатов
//! *  при разрыве канала удерживать неотправленные данные до восстановления канала связи
//! *  контролировать размер неотправленных данных, не допускать переполнения памяти

use crate::config::SharedConfig;
use crate::data::StoredResult;

use log::{error, info};
use tokio::sync::mpsc::Receiver;

/// Запускает в асинхронном режиме подсистему отправки итогового результата, полученного от inference, в систему хранения
/// 
/// Параметры:
/// 
/// **cfg** - доступ к общей разделяемой конфигурации приложения. Потокобезопасность обеспечивается самим объектом
/// 
/// **rx_rslt** - межпоточный канал получения результата из inference, читатель
/// 
/// Возвращает Future для ожидания в вызывающем коде
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
