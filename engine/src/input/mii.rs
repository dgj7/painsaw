use std::time::Instant;

#[derive(Clone, Debug)]
pub struct MouseInputInfo {
    pub when: Instant,
    pub handled: bool,
    pub x: i32,
    pub y: i32,
}
