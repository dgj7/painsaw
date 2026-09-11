use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;

///
/// a control is any ui element that can be interacted with by a user to perform some function.
///
pub trait Control: Clickable + Padded + Dimensional {
    ///
    /// produce a model.
    ///
    fn model(&self) -> Model2D;
}
