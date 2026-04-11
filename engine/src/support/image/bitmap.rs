use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Error, Seek, SeekFrom};
use std::io::ErrorKind::{InvalidData, Unsupported};
use std::path::Path;
use crate::support::image::RawImage;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

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
    log(LogLevel::Debug, &||format!("BMP DIB: dib_sz={}, width={}, height={}, planes={}, bpp={}", dib_sz, width, height, planes, bpp));

    /* prepare read pixel data */
    reader.seek(SeekFrom::Start(offset as u64))?;
    let mut bytes = vec!();
    reader.read_to_end(&mut bytes).expect("TODO: panic message");

    /* determine if there's a color table */
    let color_table_bytes = file_sz - (14 + dib_sz + (bytes.len() as u32));
    log(LogLevel::Debug, &||format!("BMP: color table bytes: {}", color_table_bytes));

    /* read the pixel data */
    if bpp == 32 {
        log(LogLevel::Debug, &||format!("BMP: bytes={}, pixels(bytes/4)={}, width*height={}", bytes.len(), bytes.len() as f32 / 4f32, width * height));
        let pixels = parse_32_bit(width, height, &mut reader);
        Ok(RawImage::new(width, height, pixels))
    } else if bpp == 24 {
        log(LogLevel::Debug, &||format!("BMP: bytes={}, pixels(bytes/3)={}, width*height={}", bytes.len(), bytes.len() as f32 / 3f32, width * height));
        let pixels = parse_24_bit(width, height, &mut reader);
        Ok(RawImage::new(width, height, pixels))
    } else {
        Err(Error::new(Unsupported, format!("bitmap: unsupported bit encoding: {}", bpp)))
    }
}

fn parse_32_bit<R: BufRead>(width: u32, height: u32, reader: &mut R) -> Vec<u8> {
    log(LogLevel::Debug, &|| "parsing bmp as 32-bit...".to_string());

    let row_size = ((24*width+31)/32)*4;
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    /* iterate over rows */
    for y in 0..height {
        /* read the row data */
        let mut row = vec![0u8; (row_size * 4) as usize];
        reader.read_exact(&mut row).expect("todo: 32-bit: row read error");

        /* calculate indices */
        let target_y = if height > 0 { height - 1 - y } else { y };
        let dest_start = target_y * width * 4;

        /* iterate over columns */
        for x in 0..width {
            /* calculate indices */
            let src_idx = (x * 4) as usize;
            let dest_idx: usize = (dest_start + (x * 4)) as usize;

            /* write pixel colors; note the encoded BGRA color scheme being corrected */
            pixels[dest_idx] = row[src_idx + 2];        // r
            pixels[dest_idx + 1] = row[src_idx + 1];    // g
            pixels[dest_idx + 2] = row[src_idx];        // b
            pixels[dest_idx + 3] = 1;                   // a
        }
    }

    /* done */
    pixels
}

fn parse_24_bit<R: BufRead>(width: u32, height: u32, reader: &mut R) -> Vec<u8> {
    log(LogLevel::Debug, &|| "parsing bmp as 24-bit...".to_string());

    let row_size = (width * 3 + 3) & !3;
    let mut pixels = vec![0u8; (row_size * 4) as usize];

    for y in 0..height {
        let mut row = vec![0u8; (row_size * 3) as usize];
        reader.read_exact(&mut row).expect("todo: 24-bit: row read error");

        let target_y = if height > 0 { height - 1 - y } else { y };
        let dest_start = target_y * width * 4;

        for x in 0..width {
            let src_idx = (x * 4) as usize;
            let dest_idx: usize = (dest_start + (x * 4)) as usize;

            pixels[dest_idx] = row[src_idx + 2];        // r
            pixels[dest_idx + 1] = row[src_idx + 1];    // g
            pixels[dest_idx + 2] = row[src_idx];        // b
            pixels[dest_idx + 3] = 1;                   // a
        }
    }

    pixels
}
