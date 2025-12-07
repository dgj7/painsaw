use crate::model::log_config::LoggingConfig;
use crate::model::log_level::LogLevel;

pub mod model;

///
/// Log a message.
/// 
pub fn log<F>(config: &LoggingConfig, level: LogLevel, message_provider: &F)
where
    F: Fn() -> &'static str,
{
    for tc in config.pairs.iter() {
        if level.is_allowed(&tc.level) {
            tc.target.print(message_provider);
        }
    }
}
