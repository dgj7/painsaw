use crate::model::log_level::LogLevel;
use crate::model::log_target::LogTarget;

#[derive(Debug,Clone)]
pub struct LogConfigPair {
    pub level: LogLevel,
    pub target: LogTarget,
}

#[derive(Debug,Clone)]
pub struct LoggerConfig {
    pub(crate) pairs: Vec<LogConfigPair>,
}

impl LoggerConfig {
    pub fn new(pairs: &[LogConfigPair]) -> LoggerConfig {
        LoggerConfig { pairs: Vec::from(pairs) }
    }
}
