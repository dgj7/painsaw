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

    pub fn get_key_info(&self) -> KeyInfo {
        match self {
            KeyPosition::KeyDown { info } => info.clone(),
            KeyPosition::KeyUp { info } => info.clone(),
        }
    }
}
