use crate::input::mii::MouseInputInfo;

pub enum MouseChange {
    Active { input: MouseInputInfo },
    Inactive { input: MouseInputInfo },
}
