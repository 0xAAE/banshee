pub enum StoredResult {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
