
///
/// read some number of bits from a byte.
///
/// for example, to read bits 2-4 (2,3,4), call:
///
/// byte_to_bits_as_u8(byte, 2, 3);
///
pub fn byte_to_bits_as_u8(byte: u8, index: u8, count: u8) -> u8 {
    (byte >> index) & count
}
