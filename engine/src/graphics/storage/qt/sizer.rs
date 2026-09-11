pub mod horizontal;

use crate::graphics::storage::qt::control::Control;
use crate::graphics::storage::qt::support::assembled::Assembled;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;
use crate::graphics::storage::qt::support::resizable::Resizable;

///
/// construct that dynamically stores the arrangement of other sizers and controls.
///
/// specifically:
/// * allows other sizers and ui elements to be contained within it
/// * can produce an ui element that has had an interaction based on coordinates
/// * can produce a list of 2d models for rendering
///
pub trait Sizer: Clickable + Padded + Dimensional + Assembled + Resizable {
    ///
    /// add a [Sizer].
    /// 
    fn add_sizer(&mut self, sizer: Box<dyn Sizer>);
    
    ///
    /// add a [Control].
    /// 
    fn add_control(&mut self, control: Box<dyn Control>);
}
