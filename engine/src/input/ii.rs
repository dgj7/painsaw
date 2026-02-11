use std::time::Instant;

#[derive(Clone, Debug)]
pub struct InputInfo {
    pub when: Instant,
    pub handled: bool,
}

impl InputInfo {
    pub fn handled() -> InputInfo {
        InputInfo {
            when: Instant::now(),
            handled: true,
        }
    }

    pub fn unhandled() -> InputInfo {
        InputInfo {
            when: Instant::now(),
            handled: false,
        }
    }
}
