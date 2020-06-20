/// Полученный от системы сопряжения фрагмент
pub enum Fragment {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
