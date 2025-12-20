pub mod log_level;
pub mod log_target;
pub mod log_config;

pub use crate::logger::log_config::LoggerConfig;
use crate::logger::log_level::LogLevel;
use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct Logger {
    pub(crate) pairs: Vec<LoggerConfig>,
}

impl Logger {
    pub fn new(pairs: &[LoggerConfig]) -> Arc<Logger> {
        Arc::from(Logger { pairs: Vec::from(pairs) })
    }

    pub fn log<F>(&self, level: LogLevel, message_provider: &F)
    where
        F: Fn() -> String,
    {
        self.pairs
            .iter()
            .filter(|x| level.is_allowed(&x.level))
            .map(|tc| (tc, message_provider()))
            .for_each(|tuple| tuple.0.target.print(&tuple.1));
    }
}
