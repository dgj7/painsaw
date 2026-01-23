#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use crate::config::create_window_config;
use demo1_world_controller::Demo1WorldController;
use engine::logger::log_level::LogLevel;
use engine::logger::log_target::LogTarget;
use engine::logger::{configure, log, LoggerConfig};
use engine::window::create_window;

pub mod demo1_world_controller;
mod config;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let cfg = create_window_config();
    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(Box::new(Demo1WorldController::new())).expect("window creation failed"); }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
