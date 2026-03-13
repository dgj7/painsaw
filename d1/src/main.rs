#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use crate::config::create_engine_config;
use demo1_world_controller::Demo1WorldController;
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::support::logger::{configure, log, LoggerConfig};
use engine::window::create_window;

pub mod demo1_world_controller;
mod config;
mod d1m2d;
mod d1m3d;
mod key_inputs;
mod mouse_inputs;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let cfg = create_engine_config();
    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(Box::new(Demo1WorldController::new()), cfg).expect("window creation failed"); }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
