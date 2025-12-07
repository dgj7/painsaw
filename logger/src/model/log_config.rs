use crate::model::log_level::LogLevel;
use crate::model::log_target::LogTarget;

#[derive(Debug,Clone)]
pub struct LogConfigPair {
    pub level: LogLevel,
    pub target: LogTarget,
}

#[derive(Debug,Clone)]
pub struct LoggingConfig {
    pub(crate) pairs: Vec<LogConfigPair>,
}

impl LoggingConfig {
    pub fn new(pairs: &[LogConfigPair]) -> LoggingConfig {
        LoggingConfig { pairs: Vec::from(pairs) }
    }
}
