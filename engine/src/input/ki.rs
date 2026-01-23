use std::time::Instant;

#[derive(Clone,Debug)]
pub struct KeyInfo {
    pub when: Instant,
    pub handled: bool,
}

impl KeyInfo {
    pub fn handled() -> KeyInfo {
        KeyInfo {
            when: Instant::now(),
            handled: true,
        }
    }

    pub fn unhandled() -> KeyInfo {
        KeyInfo {
            when: Instant::now(),
            handled: false,
        }
    }
}
