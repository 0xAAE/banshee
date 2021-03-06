/// Подготовленный к расчету результата сэмпл.
/// Передается по каналу processor --> inference.
pub enum FinalSample {
    /// Признак команды закрыть канал и завершить работу 
    Stop,
    /// Данные окончательного сэмпла
    Data {
        /// Идентификатор абонента
        id: u32,
        /// Упакованный в байтовый поток сэмпл
        value: Vec<u8>
    }
}
