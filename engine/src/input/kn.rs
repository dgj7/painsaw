use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum KeyName {
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

impl Display for KeyName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyName::KeyA => write!(f, "{}", "KeyA"),
            KeyName::KeyD => write!(f, "{}", "KeyD"),
            KeyName::KeyG => write!(f, "{}", "KeyG"),
            KeyName::KeyM => write!(f, "{}", "KeyM"),
            KeyName::KeyS => write!(f, "{}", "KeyS"),
            KeyName::KeyW => write!(f, "{}", "KeyW"),
            KeyName::MouseLeft => write!(f, "{}", "MouseLeft"),
            KeyName::MouseRight => write!(f, "{}", "MouseRight"),
            KeyName::MouseScroll => write!(f, "{}", "MouseScroll"),
            KeyName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
