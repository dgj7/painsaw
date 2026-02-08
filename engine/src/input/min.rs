use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
pub enum MouseInputName {
    MouseLeftButton,
    MouseRightButton,
    MouseScroll,
    MouseMove { x: i32, y: i32 },
}

impl Display for MouseInputName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MouseInputName::MouseLeftButton => write!(f, "{}", "MouseLeftButton"),
            MouseInputName::MouseRightButton => write!(f, "{}", "MouseRightButton"),
            MouseInputName::MouseScroll => write!(f, "{}", "MouseScroll"),
            MouseInputName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
