mod model;

use crate::model::config::ApplicationConfiguration;
use logger::model::log_config::{LogConfigPair, LoggerConfig};
use logger::model::log_level::LogLevel;
use logger::model::log_target::LogTarget;
use window::create_window;
use window::model::window_config::{WindowConfig, WindowDimensions};

fn main() {
    let cfg = configure();
    cfg.logger.info(&|| "begin");

    match create_window(&cfg.window) {
        Ok(win) => {
            win.begin_event_handling(&cfg.logger).expect("window creation failed");
        }
        Err(_e) => {
            cfg.logger.error(&|| "window creation failed");
            std::process::exit(1);
        }
    }

    cfg.logger.info(&|| "end.");
}

fn configure() -> ApplicationConfiguration {
    ApplicationConfiguration {
        logger: LoggerConfig::new(&*vec![LogConfigPair {
            level: LogLevel::Debug,
            target: LogTarget::StdOut,
        }]),
        window: WindowConfig::new(WindowDimensions::Fullscreen, "painsaw"),
    }
}
