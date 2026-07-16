use crate::input::keyboard::kii::KeyInputInfo;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum KeyChange {
    Active { info: KeyInputInfo },
    Inactive { info: KeyInputInfo },
}

impl KeyChange {
    pub fn is_active(&self) -> bool {
        match *self {
            KeyChange::Active { .. } => true,
            KeyChange::Inactive { .. } => false,
        }
    }

    pub fn is_inactive(&self) -> bool {
        match *self {
            KeyChange::Active { .. } => false,
            KeyChange::Inactive { .. } => true,
        }
    }
    
    pub fn is_handled(&self) -> bool {
        match self {
            KeyChange::Active { info } => info.handled,
            KeyChange::Inactive { info } => info.handled,
        }
    }

    pub fn set_handled(&mut self) {
        match *self {
            KeyChange::Active { ref mut info } => {info.handled = true;}
            KeyChange::Inactive { ref mut info } => {info.handled = true;}
        }
    }

    pub fn clone_key_info(&self) -> KeyInputInfo {
        match self {
            KeyChange::Active { info } => info.clone(),
            KeyChange::Inactive { info } => info.clone(),
        }
    }
}

impl Display for KeyChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyChange::Active { info: _info } => write!(f, "ACTIVE"),
            KeyChange::Inactive { info: _info } => write!(f, "INACTIVE"),
        }
    }
}
