use std::time::Instant;

#[derive(Clone,Debug)]
pub struct KeyInfo {
    pub when: Instant,
    pub handled: bool,
}
