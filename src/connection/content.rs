#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct Content {
    pub _type: String,
    pub lenght: u32,
    pub data: String,
}

impl Content {
    pub fn new(t: String,lenght: u32, data: String) -> Self {
        Self { _type: t, lenght, data }
    }
}