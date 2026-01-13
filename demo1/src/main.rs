#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use engine::logger::log_level::LogLevel;
use engine::logger::log_target::LogTarget;
use engine::logger::{configure, log, LoggerConfig};
use engine::window::window_config::{WindowConfig, WindowDimensions};
use demo1_world_controller::Demo1WorldController;
use engine::graphics::subsystem::{GraphicsSubSystem, OpenGLPipeline};
use engine::window::create_window;

pub mod demo1_world_controller;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let cfg = WindowConfig::new(
        WindowDimensions::Dimensional { width: 1920, height: 1080 },
        "Demo1 - MsWin/OpenGL",
    "PAINSAW-DEMO1",
            GraphicsSubSystem::OpenGL { pipeline: OpenGLPipeline::FixedFunction }
    );

    match create_window(&cfg) {
        Ok(mut win) => {
            let renderer = Demo1WorldController::new();
            win.begin_event_handling(&renderer).expect("window creation failed");
        }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
