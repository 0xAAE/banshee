/// Окончательный результат для передачи в систему хранения
pub enum StoredResult {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
