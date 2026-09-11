///
/// a type with dimensions
///
pub trait Dimensional {
    ///
    /// provide the recommended height.
    ///
    fn height(&self) -> f32;

    ///
    /// provide the recommended width.
    ///
    fn width(&self) -> f32;
}
