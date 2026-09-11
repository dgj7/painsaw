mod horizontal;

use crate::geometry::dim::Dimension2D;
use crate::graphics::storage::m2d::Model2DBuilder;
use crate::graphics::storage::qt::control::Control;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;

///
/// construct that dynamically stores the arrangement of other sizers and controls.
///
/// specifically:
/// * allows other sizers and ui elements to be contained within it
/// * can produce an ui element that has had an interaction based on coordinates
/// * can produce a list of 2d models for rendering
///
pub trait Sizer: Clickable + Padded + Dimensional {
    ///
    /// add a [Sizer].
    /// 
    fn add_sizer(&mut self, sizer: Box<dyn Sizer>);
    
    ///
    /// add a [Control].
    /// 
    fn add_control(&mut self, control: Box<dyn Control>);
    
    ///
    /// resize the sizer.
    /// 
    /// likely called when the window is resized, but not necessarily.
    /// 
    fn resize(&mut self, dim: &Dimension2D);

    ///
    /// assemble the sizer, as part of a 2d model.
    ///
    fn assemble(&self, builder: &mut Model2DBuilder);
}
