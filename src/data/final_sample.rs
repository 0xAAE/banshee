pub enum FinalSample {
    Stop,
    Data {
        id: u32,
        value: Vec<u8>
    }
}
