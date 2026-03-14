#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

mod config;
mod d2wc;
mod d2ki;
mod d2mi;

use engine::support::logger::{configure, log, LoggerConfig};
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::window::create_window;
use crate::config::create_engine_config;
use crate::d2wc::Demo2WorldController;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let cfg = create_engine_config();
    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(Box::new(Demo2WorldController::new()), cfg).expect("window creation failed"); }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
