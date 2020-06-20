/// Подготовленный к расчетуц результата сэмпл
pub enum FinalSample {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
