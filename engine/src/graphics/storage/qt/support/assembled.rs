use crate::graphics::storage::m2d::Model2DBuilder;

pub trait Assembled {
    ///
    /// reassemble the control, as part of a 2d model.
    ///
    fn reassemble(&self, builder: &mut Model2DBuilder);
}
