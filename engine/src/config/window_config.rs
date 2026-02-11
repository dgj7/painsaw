pub struct WindowConfig {
    pub dimensions: WindowDimensions,
    pub title: Option<String>,
    pub window_id: Option<String>,
}

pub enum WindowDimensions {
    Fullscreen,
    Dimensional { width: i32, height: i32 },
}

impl WindowConfig {
    pub fn new(dimensions: WindowDimensions, title: &str, wndclass: &str) -> WindowConfig {
        WindowConfig {
            dimensions,
            title: Some(title.to_owned()),
            window_id: Some(wndclass.to_owned()),
        }
    }
}

impl Default for WindowConfig {
    fn default() -> WindowConfig {
        WindowConfig {
            title: Some("window title".to_owned()),
            dimensions: WindowDimensions::Dimensional { width: 800, height: 600 },
            window_id: Some("PAINSAW".to_owned()),
        }
    }
}
