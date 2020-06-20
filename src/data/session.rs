/// Собранный из фрагментов звуковой сеанс, готовый для обработки
pub enum Session {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
