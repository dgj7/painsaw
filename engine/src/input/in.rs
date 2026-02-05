use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum InputName {
    /* keyboard */
    KeyA,
    KeyD,
    KeyG,
    KeyM,
    KeyS,
    KeyW,

    /* mouse */
    MouseLeft,
    MouseRight,
    MouseScroll,
    MouseMove { x: i16, y: i16, dx: i16, dy: i16 },
}

impl Display for InputName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputName::KeyA => write!(f, "{}", "KeyA"),
            InputName::KeyD => write!(f, "{}", "KeyD"),
            InputName::KeyG => write!(f, "{}", "KeyG"),
            InputName::KeyM => write!(f, "{}", "KeyM"),
            InputName::KeyS => write!(f, "{}", "KeyS"),
            InputName::KeyW => write!(f, "{}", "KeyW"),
            InputName::MouseLeft => write!(f, "{}", "MouseLeft"),
            InputName::MouseRight => write!(f, "{}", "MouseRight"),
            InputName::MouseScroll => write!(f, "{}", "MouseScroll"),
            InputName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
