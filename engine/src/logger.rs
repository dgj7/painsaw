pub mod log_level;
pub mod log_target;
pub mod log_config;
pub mod logger_type;

use std::panic::Location;
pub use crate::logger::log_config::LoggerConfig;
use crate::logger::log_level::LogLevel;
use crate::logger::logger_type::Logger;
use std::sync::{LazyLock, Mutex};

static LOGGER: LazyLock<Mutex<Logger>> = LazyLock::new(|| Mutex::new(Logger::new()));

#[track_caller]
pub fn log<F>(level: LogLevel, message_provider: &F)
where
    F: Fn() -> String,
{
    LOGGER.lock().unwrap().log(level, Location::caller(), message_provider)
}

pub fn log_caller<F>(level: LogLevel, caller: &Location, message_provider: &F)
where
    F: Fn() -> String,
{
    LOGGER.lock().unwrap().log(level, caller, message_provider);
}

pub fn configure(config: LoggerConfig) {
    LOGGER.lock().unwrap().configure(config);
}
