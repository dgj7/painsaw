mod model;

use logger::log;
use logger::model::log_config::{LogConfigPair, LoggerConfig};
use logger::model::log_level::LogLevel;
use logger::model::log_target::LogTarget;
use window::create_window;
use window::model::window_config::{WindowDimensions, WindowConfig};
use crate::model::config::ApplicationConfiguration;

fn main() {
    let cfg = configure();
    log(&cfg.logger, LogLevel::Info, &|| "begin");

    match create_window(&cfg.window) {
        Ok(win) => {
            win.begin_display(&cfg.logger);
        },
        Err(_e) => {
            log(&cfg.logger, LogLevel::Error, &|| "window creation failed");
        }
    }

    log(&cfg.logger, LogLevel::Info, &|| "end.");
}

fn configure() -> ApplicationConfiguration {
    ApplicationConfiguration {
        logger: LoggerConfig::new(&*vec!(LogConfigPair { level: LogLevel::Debug, target: LogTarget::StdOut })),
        window: WindowConfig::new(WindowDimensions::Fullscreen, "painsaw"),
    }
}
