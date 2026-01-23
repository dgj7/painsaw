use std::fmt::{Display, Formatter};
use crate::input::ki::KeyInfo;

#[derive(Clone,Debug)]
pub enum KeyPosition {
    KeyDown { info: KeyInfo },
    KeyUp { info: KeyInfo },
}

impl KeyPosition {
    pub fn is_down(&self) -> bool {
        match *self {
            KeyPosition::KeyDown { .. } => true,
            KeyPosition::KeyUp { .. } => false,
        }
    }

    pub fn is_up(&self) -> bool {
        match *self {
            KeyPosition::KeyDown { .. } => false,
            KeyPosition::KeyUp { .. } => true,
        }
    }
    
    pub fn is_handled(&self) -> bool {
        match self {
            KeyPosition::KeyDown { info } => info.handled,
            KeyPosition::KeyUp { info } => info.handled,
        }
    }

    pub fn set_handled(&mut self) {
        match *self {
            KeyPosition::KeyDown { ref mut info } => {info.handled = true;}
            KeyPosition::KeyUp { ref mut info } => {info.handled = true;}
        }
    }

    pub fn clone_key_info(&self) -> KeyInfo {
        match self {
            KeyPosition::KeyDown { info } => info.clone(),
            KeyPosition::KeyUp { info } => info.clone(),
        }
    }
}

impl Display for KeyPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyPosition::KeyDown { info: _info } => write!(f, "DOWN"),
            KeyPosition::KeyUp { info: _info } => write!(f, "UP"),
        }
    }
}
