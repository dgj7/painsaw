pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl RawImage {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> RawImage {
        RawImage {
            width,
            height,
            data,
        }
    }
}
