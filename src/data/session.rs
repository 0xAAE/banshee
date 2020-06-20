pub enum Session {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
