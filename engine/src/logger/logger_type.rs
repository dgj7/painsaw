use crate::logger::log_level::LogLevel;
use crate::logger::LoggerConfig;
use chrono::{DateTime, Utc};
use std::panic::Location;

#[derive(Debug, Clone)]
pub struct Logger {
    pub(crate) pairs: Vec<LoggerConfig>,
}

impl Logger {
    pub const fn new() -> Logger {
        Logger { pairs: vec![] }
    }

    pub fn configure(&mut self, config: LoggerConfig) {
        self.pairs.push(config);
    }

    pub fn log<F>(&self, level: LogLevel, caller: &Location, message_provider: &F)
    where
        F: Fn() -> String,
    {
        self.pairs
            .iter()
            .filter(|x| level.is_allowed(&x.level))
            .map(|tc| (tc, message_provider()))
            .for_each(|tuple| tuple.0.target.print(&level, DateTime::from(Utc::now()), caller, &tuple.1));
    }
}
