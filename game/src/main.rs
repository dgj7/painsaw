#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use engine::logger::log_level::LogLevel;
use engine::logger::log_target::LogTarget;
use engine::logger::{Logger, LoggerConfig};
use engine::app::config::app_config::ApplicationConfiguration;
use engine::window::model::window_config::{WindowConfig, WindowDimensions};
use engine::window::window_factory::create_window;
use crate::impls::game_renderer::GameRenderer;

pub mod impls;

fn main() {
    let cfg = configure();
    cfg.logger.log(LogLevel::Info, &|| "begin".parse().unwrap());

    match create_window(&cfg.window, &cfg.logger) {
        Ok(mut win) => {
            let renderer = GameRenderer::new();
            win.begin_event_handling(&renderer).expect("window creation failed");
        }
        Err(_e) => {
            cfg.logger.log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    cfg.logger.log(LogLevel::Info, &|| "end.".parse().unwrap());
}

fn configure() -> ApplicationConfiguration {
    ApplicationConfiguration {
        logger: Logger::new(&*vec![LoggerConfig {
            level: LogLevel::Debug,
            target: LogTarget::StdOut,
        }]),
        window: WindowConfig::new(WindowDimensions::Dimensional { width: 800, height: 600 }, "painsaw"),
    }
}
