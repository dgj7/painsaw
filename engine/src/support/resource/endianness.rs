#[allow(dead_code)] // todo: remove this
enum Endianness {
    ///
    /// the "little end" (least-significant byte) is stored first.
    ///
    /// dominant in pcs (intel/amd x86, arm).
    ///
    LittleEndian,

    ///
    /// the "big end" (most-significant byte) is stored first.
    ///
    /// dominant in mainframes and network protocols (network byte order).
    ///
    BigEndian,
}
