use crate::support::logger::log_level::LogLevel;
use crate::support::logger::log_target::LogTarget;

#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub target: LogTarget,
}
