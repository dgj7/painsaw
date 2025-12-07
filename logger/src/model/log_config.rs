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

    pub fn error<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        self.log(LogLevel::Error, message_provider);
    }

    pub fn warning<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        self.log(LogLevel::Warning, message_provider);
    }

    pub fn info<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        self.log(LogLevel::Info, message_provider);
    }

    pub fn debug<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        self.log(LogLevel::Debug, message_provider);
    }

    pub fn trace<F>(&self, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        self.log(LogLevel::Trace, message_provider);
    }

    fn log<F>(&self, level: LogLevel, message_provider: &F)
    where
        F: Fn() -> &'static str,
    {
        for tc in self.pairs.iter() {
            if level.is_allowed(&tc.level) {
                tc.target.print(message_provider);
            }
        }
    }
}
