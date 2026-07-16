use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum KeyInputName {
    KeyEscape,
    KeyA,
    KeyD,
    KeyG,
    KeyM,
    KeyS,
    KeyW,
}

impl Display for KeyInputName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyInputName::KeyEscape => write!(f, "esc"),
            KeyInputName::KeyA => write!(f, "{}", "KeyA"),
            KeyInputName::KeyD => write!(f, "{}", "KeyD"),
            KeyInputName::KeyG => write!(f, "{}", "KeyG"),
            KeyInputName::KeyM => write!(f, "{}", "KeyM"),
            KeyInputName::KeyS => write!(f, "{}", "KeyS"),
            KeyInputName::KeyW => write!(f, "{}", "KeyW"),
        }
    }
}
