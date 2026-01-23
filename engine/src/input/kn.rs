use std::fmt::{Display, Formatter};

#[derive(Debug,Clone)]
#[derive(Eq, Hash, PartialEq)]
pub enum KeyName {
    /* keyboard */
    KeyG,
    KeyM,

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
            KeyName::MouseLeft => write!(f, "{}", "MouseLeft"),
            KeyName::MouseRight => write!(f, "{}", "MouseRight"),
            KeyName::MouseScroll => write!(f, "{}", "MouseScroll"),
            KeyName::MouseMove { .. } => write!(f, "{}", "MouseMove"),
        }
    }
}
