pub struct WindowConfig {
    pub dimensions: WindowDimensions,
    pub title: Option<String>,
    pub wndclass: Option<String>,
}

pub enum WindowDimensions {
    Fullscreen,
    Dimensional{ width: i32, height: i32 },
}

impl WindowConfig {
    pub fn new(dimensions: WindowDimensions, title: &str, wndclass: &str) -> WindowConfig {
        WindowConfig { dimensions, title: Some(title.to_owned()), wndclass: Some(wndclass.to_owned()) }
    }
}
