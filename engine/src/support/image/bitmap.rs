use crate::support::image::RawImage;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;
use std::fs::File;
use std::io::ErrorKind::{InvalidData, Unsupported};
use std::io::{BufRead, BufReader, Cursor, Error, Seek, SeekFrom};
use std::path::Path;

///
/// windows bitmap file format (win32: BITMAPFILEHEADER).
///
/// bitmaps are 24-bit (no alpha channel), with colors BGR.
///
#[allow(dead_code)]// todo: remove this eventually
pub fn load_bitmap_from_path<P: AsRef<Path>>(path : P) -> std::io::Result<RawImage> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    load_bitmap_from_buf_read(&mut reader)
}

pub fn load_bitmap_from_bytes(bytes: &[u8]) -> std::io::Result<RawImage> {
    let cursor = Cursor::new(bytes);
    load_bitmap_from_buf_read(cursor)
}

pub fn load_bitmap_from_buf_read<R: BufRead + Seek>(mut reader: R) -> std::io::Result<RawImage> {
    /* extract header data */
    let mut header = [0u8;14];
    reader.read_exact(&mut header)?;

    /* extract header data */
    let magic: [u8;2] = header[0..2].try_into().unwrap();
    let file_sz = u32::from_le_bytes(header[2..6].try_into().unwrap());
    let reserved = u32::from_le_bytes(header[6..10].try_into().unwrap());
    let offset = u32::from_le_bytes(header[10..14].try_into().unwrap());
    log(LogLevel::Debug, &||format!("BMP Header: magic={:?}, file_sz={}, reserved={}, offset={}", magic, file_sz, reserved, offset));

    /* first 2 bytes should be BM; otherwise, not a bitmap */
    if magic != *b"BM" {
        return Err(Error::new(InvalidData, "not a bitmap!"));
    }

    /* read dib */
    let mut dib = [0u8; 40];
    reader.read_exact(&mut dib)?;

    /* extract image details */
    let dib_sz = u32::from_le_bytes(dib[0..4].try_into().unwrap());
    let width =  u32::from_le_bytes(dib[4..8].try_into().unwrap());
    let height = u32::from_le_bytes(dib[8..12].try_into().unwrap());
    let planes = u16::from_le_bytes(dib[12..14].try_into().unwrap());
    let bpp = u16::from_le_bytes(dib[14..16].try_into().unwrap());
    log(LogLevel::Debug, &||format!("BMP: DIB: dib_sz={}, width={}, height={}, planes={}, bpp={}", dib_sz, width, height, planes, bpp));

    /* prepare read pixel data */
    reader.seek(SeekFrom::Start(offset as u64))?;
    let mut bytes = vec!();
    reader.read_to_end(&mut bytes).expect("TODO: panic message");

    /* determine if there's a color table */
    let color_table_bytes = file_sz - (14 + dib_sz + (bytes.len() as u32));
    log(LogLevel::Debug, &||format!("BMP: color table bytes: {}", color_table_bytes));

    /* read the pixel data */
    if bpp == 24 {
        log(LogLevel::Debug, &||format!("BMP: 24b: bytes={}, pixels(bytes/3)={}, width*height={}", bytes.len(), bytes.len() as f32 / 3f32, width * height));
        let pixels = parse_24_bit(width, height, bytes);
        log(LogLevel::Debug, &|| "BMP: 24b: success".to_string());
        Ok(RawImage::new(width, height, pixels))
    } else {
        Err(Error::new(Unsupported, format!("BMP: unsupported bit encoding: {}", bpp)))
    }
}

fn parse_24_bit(width: u32, height: u32, bytes: Vec<u8>) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);

    for chunk in bytes.chunks(3) {
        if chunk.len() == 3 {
            pixels.push(chunk[2]);      // BGR: red
            pixels.push(chunk[1]);      // BGR: green
            pixels.push(chunk[0]);      // BGR: blue
            pixels.push(255);           // we're inventing our own alpha channel; could also be programmable
        } else {
            log(LogLevel::Warning, &|| format!("{} bytes remain: {:?}", chunk.len(), chunk));
        }
    }

    pixels
}
