pub struct WindowRequest {
    // todo: these fields don't need to be pub; pub can be removed after we construct the actual window
    pub dimensions: WindowDimensions,
    pub title: Option<String>,
}

pub enum WindowDimensions {
    Fullscreen,
    Dimensional(u16, u16),
}

impl WindowRequest {
    pub fn new(dimensions: WindowDimensions, title: &str) -> WindowRequest {
        WindowRequest { dimensions, title: Some(title.to_owned()) }
    }
}
