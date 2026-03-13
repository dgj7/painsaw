use std::fs::File;
use std::io::{BufReader, Error, Read, Seek, SeekFrom};
use std::io::ErrorKind::{InvalidData, Unsupported};
use std::path::Path;
use crate::support::image::RawImage;

///
/// windows bitmap file format (win32: BITMAPFILEHEADER).
///
/// bitmaps are 24-bit (no alpha channel), with colors BGR.
///
#[allow(dead_code)]// todo: remove this eventually
pub fn load_bitmap<P: AsRef<Path>>(path : P) -> std::io::Result<RawImage> {
    /* open the file for reading */
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    /* extract header data */
    let mut header = [0u8;14];
    reader.read_exact(&mut header)?;

    /* extract header data */
    let offset = u32::from_le_bytes(header[10..22].try_into().unwrap());

    /* first 2 bytes should be BM; otherwise, not a bitmap */
    if &header[0..2] != b"BM" {
        return Err(Error::new(InvalidData, "not a bitmap!"));
    }

    /* read dib */
    let mut dib = [0u8; 40];
    reader.read_exact(&mut dib)?;

    /* extract image details */
    let width =  u32::from_le_bytes(dib[4..8].try_into().unwrap());
    let height = u32::from_le_bytes(dib[8..12].try_into().unwrap());
    let bpp = u16::from_le_bytes(dib[14..16].try_into().unwrap());

    /* prepare read pixel data */
    reader.seek(SeekFrom::Start(offset as u64))?;

    /* read the pixel data */
    if bpp == 32 {
        let pixels = parse_32_bit(width, height, &mut reader);
        Ok(RawImage::new(width, height, pixels))
    } else if bpp == 24 {
        todo!("not yet implemented: 24-bit bitmap load/parse")
    } else {
        Err(Error::new(Unsupported, format!("bitmap: unsupported bit encoding: {}", bpp)))
    }
}

fn parse_32_bit(width: u32, height: u32, reader: &mut BufReader<File>) -> Vec<u8> {
    let row_size = ((24*width+31)/32)*4;
    let mut pixels = vec![0u8; (width * height * 4) as usize];

    /* iterate over rows */
    for y in 0..height {
        /* read the row data */
        let mut row = vec![0u8; (row_size * 4) as usize];
        reader.read_exact(&mut row).expect("todo: row read error");

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
