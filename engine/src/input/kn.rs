use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum KeyName {
    /* keyboard */
    KeyG,
    KeyM,
    KeyW,
    KeyA,
    KeyS,
    KeyD,

    /* mouse */
    MouseLeft,
    MouseRight,
    MouseScroll,
    MouseMove { x: i16, y: i16, dx: i16, dy: i16 },
}

impl Display for KeyName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyName::KeyG => write!(f, "{}", "KeyG"),
            KeyName::KeyM => write!(f, "{}", "KeyM"),
            KeyName::KeyW => write!(f, "{}", "KeyW"),
            KeyName::KeyA => write!(f, "{}", "KeyA"),
            KeyName::KeyS => write!(f, "{}", "KeyS"),
            KeyName::KeyD => write!(f, "{}", "KeyD"),
            KeyName::MouseLeft => write!(f, "{}", "MouseLeft"),
            KeyName::MouseRight => write!(f, "{}", "MouseRight"),
            KeyName::MouseScroll => write!(f, "{}", "MouseScroll"),
            KeyName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
