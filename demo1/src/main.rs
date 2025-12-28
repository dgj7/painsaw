#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use engine::logger::log_level::LogLevel;
use engine::logger::log_target::LogTarget;
use engine::logger::{configure, log, LoggerConfig};
use engine::window::model::window_config::{WindowConfig, WindowDimensions};
use engine::window::window_factory::create_window;
use demo1_renderer::Demo1Renderer;

pub mod demo1_renderer;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let cfg = WindowConfig::new(
        WindowDimensions::Dimensional { width: 800, height: 600 },
        "Demo1 - MsWin/OpenGL",
    "PAINSAW-DEMO1");

    match create_window(&cfg) {
        Ok(mut win) => {
            let renderer = Demo1Renderer::new();
            win.begin_event_handling(&renderer).expect("window creation failed");
        }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
