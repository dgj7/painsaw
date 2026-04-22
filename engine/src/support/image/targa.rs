use crate::support::image::{Image, RawImage};
use std::io::{BufRead, Error, Seek, SeekFrom};
use std::io::ErrorKind::Unsupported;
use crate::support::binary::byte_to_bits_as_u8;
use crate::support::logger::log;
use crate::support::logger::log_level::LogLevel;

pub struct Targa;

const HEADER_LEN : usize = 18;
const FOOTER_IDX : usize = 26;

#[repr(C)]
struct TargaFooter {
    extension_offset: u32,
    developer_offset: u32,
    marker: [u8;16],
    reserved: u8,
    terminator: u8,
}

///
/// load targa image files.
///
impl Image for Targa {
    fn load_from_buf_read<R: BufRead + Seek>(mut reader: R) -> std::io::Result<RawImage> {
        /* determine the file size; reset the stream to start */
        let file_size = reader.seek(SeekFrom::End(0))?;
        log(LogLevel::Debug, &||format!("TGA: file_size={}", file_size));

        /* load the footer */
        let maybe_footer = load_footer(&mut reader);
        let mut footer_bytes_consumed = 0;
        if maybe_footer.is_some() {
            footer_bytes_consumed = FOOTER_IDX;
            let footer = maybe_footer.unwrap();
            if footer.developer_offset > 0 {
                todo!("TGA: Footer: handle developer offset")
            }
            if footer.extension_offset > 0 {
                todo!("TGA: Footer: handle extension offset")
            }
        }
        reader.seek(SeekFrom::Start(0))?;

        /* load the header */
        let mut header = [0u8; HEADER_LEN];
        reader.read_exact(&mut header)?;

        /* extract header fields */
        let id_len = header[0];
        let color_map_type = header[1];                                                             // 0=no color map, 1=color map
        let image_type = header[2];                                                                 // 0=empty, 1=uncompressed color-mapped, 2=uncompressed truecolor, 3=uncompressed b/w, 9=rle colormapped, 10=rletruecolor (compressed), 11=rle b/w
        let color_map_spec: [u8;5] = header[3..8].try_into().unwrap();                              // 2b=color map origin, 2b=color map length, 1b=color map size
        let x_origin = u16::from_le_bytes(header[8..10].try_into().unwrap());
        let y_origin = u16::from_le_bytes(header[10..12].try_into().unwrap());
        let width = u16::from_le_bytes(header[12..14].try_into().unwrap());
        let height = u16::from_le_bytes(header[14..16].try_into().unwrap());
        let pixel_depth = header[16];
        let image_desc = header[17];
        log(LogLevel::Debug, &||format!("TGA: id_len={}, color_map_type={}, image_type={}, color_map_spec={:?}, x_origin={}, y_origin={}, width={}, height={}, pixel_depth={}, image_desc={}", id_len, color_map_type, image_type, color_map_spec, x_origin, y_origin, width, height, pixel_depth, image_desc));
        let img_desc_alpha_channel_depth = byte_to_bits_as_u8(image_desc, 0, 4);                    // 0=no alpha, 8=32-bit/8-bit alpha, 1=16-bit/1-bit alpha
        let img_desc_left_to_right = byte_to_bits_as_u8(image_desc, 4, 1);                          // 0=left-to-right, 1=right-to-left
        let img_desc_bottom_to_top = byte_to_bits_as_u8(image_desc, 5, 1);                          // 0=bottom-to-top, 1=top-to-bottom
        let img_desc_reserved = byte_to_bits_as_u8(image_desc, 6, 2);
        log(LogLevel::Debug, &|| format!("TGA: image_desc: alpha_channel_depth={}, left_to_right={}, top_to_bottom={}, reserved={}", img_desc_alpha_channel_depth, img_desc_left_to_right, img_desc_bottom_to_top, img_desc_reserved));

        /* color map type:  */
        if color_map_type > 0 {
            return Err(Error::new(Unsupported, format!("TGA: unsupported color map type {}", color_map_type)))
        }

        /* currently only support 2, uncompressed truecolor */
        if image_type != 2 {
            return Err(Error::new(Unsupported, format!("TGA: unsupported image type {}", image_type)))
        }

        /* calculate where image data starts */
        let start = HEADER_LEN + id_len as usize;
        let data_size = (file_size - start as u64) - footer_bytes_consumed as u64;
        log(LogLevel::Debug, &||format!("TGA: data_size={}", data_size));

        /* load the image data, stored in bgr/bgra */
        let curr_stream_pos = reader.stream_position()?;
        log(LogLevel::Debug, &|| format!("TGA: current file pointer position: {}", curr_stream_pos));
        let mut data = vec![0u8; data_size as usize];
        reader.read_exact(&mut data)?;

        /* parse the image data based on characteristics of the image */
        if pixel_depth == 32 {
            let pixels = parse_32_bit(width, height, data, img_desc_left_to_right == 0, img_desc_bottom_to_top != 0);
            Ok(RawImage::new(width as u32, height as u32, pixels))
        } else {
            Err(Error::new(Unsupported, format!("TGA: unsupported pixel depth: {}", pixel_depth)))
        }
    }
}

fn parse_32_bit(width: u16, height: u16, bytes: Vec<u8>, left_to_right: bool, top_to_bottom: bool) -> Vec<u8> {
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    let chunk_sz = 4;
    let row_sz = bytes.len() / height as usize;

    if top_to_bottom {
        for _pixel in bytes.chunks(chunk_sz) {
            if left_to_right {
                todo!("TGA: top-to-bottom: implement left-to-right")
            } else {
                todo!("TGA: top-to-bottom: implement right-to-left")
            }
        }
    } else {
        for row in bytes.rchunks(row_sz) {
            for pixel in row.chunks(chunk_sz) {
                if pixel.len() == chunk_sz {
                    if left_to_right {
                        pixels.push(pixel[2]);      // BGRA: red
                        pixels.push(pixel[1]);      // BGRA: green
                        pixels.push(pixel[0]);      // BGRA: blue
                        pixels.push(pixel[3]);
                    } else {
                        todo!("TGA: bottom-to-top: implement right-to-left")
                    }
                } else {
                    log(LogLevel::Warning, &|| format!("TGA: bottom-to-top: chunk_length={}", pixel.len()));
                }
            }
        }
    }

    pixels
}

fn load_footer<R: BufRead + Seek>(mut reader: R) -> Option<TargaFooter> {
    /* load footer bytes */
    reader.seek(SeekFrom::End(-26)).expect("TODO: panic message");
    let mut footer = [0u8; FOOTER_IDX];
    reader.read_exact(&mut footer).expect("TODO: panic message");

    /* read the marker */
    let marker: [u8;16] = footer[8..24].try_into().unwrap();

    /* if footer marker matches, fill fields and return */
    if marker == *b"TRUEVISION-XFILE" {
        /* read additional data fields */
        let extension_offset = u32::from_le_bytes(footer[0..4].try_into().unwrap());
        let developer_offset = u32::from_le_bytes(footer[4..8].try_into().unwrap());
        let reserved: u8 = footer[24].try_into().unwrap();
        let terminator: u8 = footer[25].try_into().unwrap();

        /* print something so it's obvious we've found a footer */
        log(LogLevel::Debug, &|| format!("TGA: Footer: extension_offset={}, developer_offset={}, marker={:?}, reserved={}, terminator={}", extension_offset, developer_offset, str::from_utf8(&marker), reserved, terminator));

        /* done */
        return Some(TargaFooter {
            extension_offset,
            developer_offset,
            marker,
            reserved,
            terminator,
        })
    }

    /* no footer */
    None
}
