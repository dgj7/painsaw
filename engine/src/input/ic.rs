use crate::input::ii::InputInfo;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum InputChange {
    Active { info: InputInfo },
    Inactive { info: InputInfo },
}

impl InputChange {
    pub fn is_active(&self) -> bool {
        match *self {
            InputChange::Active { .. } => true,
            InputChange::Inactive { .. } => false,
        }
    }

    pub fn is_inactive(&self) -> bool {
        match *self {
            InputChange::Active { .. } => false,
            InputChange::Inactive { .. } => true,
        }
    }
    
    pub fn is_handled(&self) -> bool {
        match self {
            InputChange::Active { info } => info.handled,
            InputChange::Inactive { info } => info.handled,
        }
    }

    pub fn set_handled(&mut self) {
        match *self {
            InputChange::Active { ref mut info } => {info.handled = true;}
            InputChange::Inactive { ref mut info } => {info.handled = true;}
        }
    }

    pub fn clone_key_info(&self) -> InputInfo {
        match self {
            InputChange::Active { info } => info.clone(),
            InputChange::Inactive { info } => info.clone(),
        }
    }
}

impl Display for InputChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputChange::Active { info: _info } => write!(f, "ACTIVE"),
            InputChange::Inactive { info: _info } => write!(f, "INACTIVE"),
        }
    }
}
