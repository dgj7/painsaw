use crate::graphics::storage::qt::support::assembled::Assembled;
use crate::graphics::storage::qt::support::clickable::Clickable;
use crate::graphics::storage::qt::support::dimensional::Dimensional;
use crate::graphics::storage::qt::support::padded::Padded;
use crate::graphics::storage::qt::support::resizable::Resizable;

///
/// a control is any ui element that can be interacted with by a user to perform some function.
///
pub trait Control: Clickable + Padded + Dimensional + Assembled + Resizable {}
