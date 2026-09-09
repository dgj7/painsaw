use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::clickable::Clickable;

///
/// a control is any ui element that can be interacted with by a user to perform some function.
///
pub trait Control: Clickable {
    ///
    /// produce a model.
    ///
    fn model(&self) -> Model2D;
}
