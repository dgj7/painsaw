pub struct WindowConfig {
    // todo: these fields don't need to be pub; pub can be removed after we construct the actual window
    pub dimensions: WindowDimensions,
    pub title: Option<String>,
}

pub enum WindowDimensions {
    Fullscreen,
    Dimensional{ width: i32, height: i32 },
}

impl WindowConfig {
    pub fn new(dimensions: WindowDimensions, title: &str) -> WindowConfig {
        WindowConfig { dimensions, title: Some(title.to_owned()) }
    }
}
