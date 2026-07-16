use std::time::Instant;

#[derive(Clone, Debug)]
pub struct KeyInputInfo {
    pub when: Instant,
    pub handled: bool,
}

impl KeyInputInfo {
    pub fn handled() -> KeyInputInfo {
        KeyInputInfo {
            when: Instant::now(),
            handled: true,
        }
    }

    pub fn unhandled() -> KeyInputInfo {
        KeyInputInfo {
            when: Instant::now(),
            handled: false,
        }
    }
}
