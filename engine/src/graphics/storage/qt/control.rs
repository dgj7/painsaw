use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::clickable::Clickable;
use crate::graphics::storage::qt::padded::Padded;

///
/// a control is any ui element that can be interacted with by a user to perform some function.
///
pub trait Control: Clickable + Padded {
    ///
    /// produce a model.
    ///
    fn model(&self) -> Model2D;

    ///
    /// provide the recommended height.
    ///
    fn height(&self) -> f32;

    ///
    /// provide the recommended width.
    ///
    fn width(&self) -> f32;
}
