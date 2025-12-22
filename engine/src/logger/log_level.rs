
#[derive(Debug, Clone)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    fn priority(&self) -> u8 {
        match *self {
            LogLevel::Error => 5,
            LogLevel::Warning => 4,
            LogLevel::Info => 3,
            LogLevel::Debug => 2,
            LogLevel::Trace => 1,
        }
    }

    // todo: unit test this
    pub(crate) fn is_allowed(&self, configured_level: &LogLevel) -> bool {
        configured_level.priority() <= self.priority()
    }
}
