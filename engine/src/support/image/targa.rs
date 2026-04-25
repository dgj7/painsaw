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
#[derive(Debug)]
struct TargaMetaData {
    /* header info */
    file_size: u64,
    id_len: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_spec: [u8;5],
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,

    /* image desc */
    image_desc: u8,
    alpha_channel_depth: u8,
    left_to_right: bool,
    top_to_bottom: bool,
    reserved: u8,

    /* image data */
    data_start: usize,
    data_size: u64,
}

#[repr(C)]
#[derive(Debug)]
struct TargaFooter {
    extension_offset: u32,
    developer_offset: u32,
    marker: [u8;16],
    reserved: u8,
    terminator: u8,
    dev_sz: u32,
    ext_sz: u32,
}

///
/// load targa image files.
///
impl Image for Targa {
    fn load_from_buf_read<R: BufRead + Seek>(mut reader: R) -> std::io::Result<RawImage> {
        /* load the metadata */
        log(LogLevel::Debug, &|| "TGA: begin----------------".to_string());
        let metadata = load_metadata(&mut reader);
        log(LogLevel::Debug, &|| format!("TGA: {:?}", metadata));
        validate(&metadata);

        /* load the image data, stored in bgr/bgra */
        let mut data = vec![0u8; metadata.data_size as usize];
        reader.read_exact(&mut data)?;

        /* parse the image data based on characteristics of the image */
        if metadata.pixel_depth == 32 {
            let pixels = parse_32_bit(&metadata, data);
            Ok(RawImage::new(metadata.width as u32, metadata.height as u32, pixels))
        } else {
            Err(Error::new(Unsupported, format!("TGA: unsupported pixel depth: {}", metadata.pixel_depth)))
        }
    }
}

fn parse_32_bit(metadata: &TargaMetaData, bytes: Vec<u8>) -> Vec<u8> {
    let mut pixels = Vec::with_capacity(metadata.width as usize * metadata.height as usize * 4);
    let chunk_sz = 4;
    let row_sz = bytes.len() / metadata.height as usize;

    if metadata.top_to_bottom {
        for _pixel in bytes.chunks(chunk_sz) {
            if metadata.left_to_right {
                todo!("TGA: top-to-bottom: implement left-to-right")
            } else {
                todo!("TGA: top-to-bottom: implement right-to-left")
            }
        }
    } else {
        for row in bytes.rchunks(row_sz) {
            if metadata.left_to_right {
                for pixel in row.chunks(chunk_sz) {
                    if pixel.len() == chunk_sz {
                        pixels.push(pixel[2]);      // BGRA: red
                        pixels.push(pixel[1]);      // BGRA: green
                        pixels.push(pixel[0]);      // BGRA: blue
                        pixels.push(pixel[3]);
                    } else {
                        log(LogLevel::Warning, &|| format!("TGA: bottom-to-top: chunk_length={}", pixel.len()));
                    }
                }
            } else {
                todo!("TGA: bottom-to-top: implement right-to-left")
            }
        }
    }

    pixels
}

fn validate(metadata: &TargaMetaData) {
    /* color map type:  */
    if metadata.color_map_type > 0 {
        panic!("TGA: unsupported color map type {}", metadata.color_map_type);
    }

    /* currently only support 2, uncompressed truecolor */
    if metadata.image_type != 2 {
        panic!("TGA: unsupported image type {}", metadata.image_type);
    }
}

fn load_metadata<R: BufRead + Seek>(mut reader: R) -> TargaMetaData {
    /* determine the file size; reset the stream to start */
    let file_size = reader.seek(SeekFrom::End(0)).expect("TODO: panic message");

    /* load the footer */
    let maybe_footer = load_footer(&mut reader);
    let mut footer_bytes_consumed = 0;
    if maybe_footer.is_some() {
        footer_bytes_consumed = FOOTER_IDX;
        let footer = maybe_footer.unwrap();
        footer_bytes_consumed += footer.dev_sz as usize;
        footer_bytes_consumed += footer.ext_sz as usize;
    }
    reader.seek(SeekFrom::Start(0)).expect("TODO: panic message");

    /* load the header */
    let mut header = [0u8; HEADER_LEN];
    reader.read_exact(&mut header).expect("TODO: panic message");

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
    let alpha_channel_depth = byte_to_bits_as_u8(image_desc, 0, 4);                    // 0=no alpha, 8=32-bit/8-bit alpha, 1=16-bit/1-bit alpha
    let left_to_right = byte_to_bits_as_u8(image_desc, 4, 1);                          // 0=left-to-right, 1=right-to-left
    let bottom_to_top = byte_to_bits_as_u8(image_desc, 5, 1);                          // 0=bottom-to-top, 1=top-to-bottom
    let reserved = byte_to_bits_as_u8(image_desc, 6, 2);

    /* calculate where image data starts */
    let data_start = HEADER_LEN + id_len as usize;
    let data_size = (file_size - data_start as u64) - footer_bytes_consumed as u64;

    /* done */
    TargaMetaData {
        file_size,

        id_len,
        color_map_type,
        image_type,
        color_map_spec,
        x_origin,
        y_origin,
        width,
        height,
        pixel_depth,

        image_desc,
        alpha_channel_depth,
        left_to_right: left_to_right == 0,
        top_to_bottom: bottom_to_top != 0,
        reserved,

        data_start,
        data_size,
    }
}

fn load_footer<R: BufRead + Seek>(mut reader: R) -> Option<TargaFooter> {
    /* load footer bytes */
    reader.seek(SeekFrom::End(-26)).expect("TODO: panic message");
    let footer_begin = reader.stream_position().unwrap();

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

        /* load up the developer and extension data, if present */
        let ext_sz: u32 = if extension_offset != 0 {
            if developer_offset != 0 {
                footer_begin as u32 - developer_offset.abs_diff(extension_offset)
            } else {
                footer_begin as u32 - extension_offset
            }
        } else { 0 };
        let dev_sz: u32 = if developer_offset != 0 {
            if extension_offset != 0 {
                footer_begin as u32 - extension_offset.abs_diff(developer_offset)
            } else {
                footer_begin as u32 - developer_offset
            }
        } else { 0 };

        /* done */
        let footer = TargaFooter {
            extension_offset,
            developer_offset,
            marker,
            reserved,
            terminator,
            dev_sz,
            ext_sz,
        };
        log(LogLevel::Debug, &|| format!("TGA: {:?}", footer));
        return Some(footer);
    }

    /* no footer */
    None
}
