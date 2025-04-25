#[derive(Debug,Clone)]
pub struct Content {
    pub _type: String,
    pub lenght: u32,
    pub data: Vec<u8>,
}

impl Content {
    pub fn new(t: String,lenght: u32, data: Vec<u8>) -> Self {
        Self { _type: t, lenght, data }
    }
}