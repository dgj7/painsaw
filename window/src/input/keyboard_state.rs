use std::time::Instant;

#[derive(Clone,Debug)]
pub enum KeyState {
    KeyDown(Instant),
    KeyUp(),
}
