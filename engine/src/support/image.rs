use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek};
use std::path::Path;

pub mod bitmap;
pub mod targa;
pub mod tiff;

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

pub trait Image {
    fn load_from_buf_read<R: BufRead + Seek>(reader: R) -> std::io::Result<RawImage>;

    fn load_from_path<P: AsRef<Path>>(path : P) -> std::io::Result<RawImage> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::load_from_buf_read(&mut reader)
    }

    fn load_from_bytes(bytes: &[u8]) -> std::io::Result<RawImage> {
        let cursor = Cursor::new(bytes);
        Self::load_from_buf_read(cursor)
    }
}
