use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum InputName {
    /* keyboard */
    KeyEscape,
    KeyA,
    KeyD,
    KeyG,
    KeyM,
    KeyS,
    KeyW,

    /* mouse */
    MouseLeftButton,
    MouseRightButton,
    MouseScroll,
    MouseMove { x: i32, y: i32 },
}

impl Display for InputName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputName::KeyEscape => write!(f, "esc"),
            InputName::KeyA => write!(f, "{}", "KeyA"),
            InputName::KeyD => write!(f, "{}", "KeyD"),
            InputName::KeyG => write!(f, "{}", "KeyG"),
            InputName::KeyM => write!(f, "{}", "KeyM"),
            InputName::KeyS => write!(f, "{}", "KeyS"),
            InputName::KeyW => write!(f, "{}", "KeyW"),
            InputName::MouseLeftButton => write!(f, "{}", "MouseLeftButton"),
            InputName::MouseRightButton => write!(f, "{}", "MouseRightButton"),
            InputName::MouseScroll => write!(f, "{}", "MouseScroll"),
            InputName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
