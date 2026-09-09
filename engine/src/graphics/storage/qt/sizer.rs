use crate::geometry::dim::Dimension2D;
use crate::graphics::storage::m2d::Model2D;
use crate::graphics::storage::qt::clickable::Clickable;
use crate::graphics::storage::qt::control::Control;
use crate::graphics::storage::qt::padded::Padded;

///
/// construct that dynamically stores the arrangement of other sizers and controls.
///
/// specifically:
/// * allows other sizers and ui elements to be contained within it
/// * can produce an ui element that has had an interaction based on coordinates
/// * can produce a list of 2d models for rendering
///
pub trait Sizer: Clickable + Padded {
    ///
    /// add a [Sizer].
    /// 
    fn add_sizer(&mut self, sizer: dyn Sizer);
    
    ///
    /// add a [Control].
    /// 
    fn add_control(&mut self, control: dyn Control);
    
    ///
    /// resize the sizer.
    /// 
    /// likely called when the window is resized, but not necessarily.
    /// 
    fn resize(&mut self, dim: &Dimension2D);

    ///
    /// model the sizer and/or it's contents.
    ///
    fn model(&self) -> &[Model2D];
}
