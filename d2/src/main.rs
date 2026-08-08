#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

mod d2ec;
mod d2wc;
mod d2kh;
mod d2mh;
mod d2m2d;
mod d2;

use std::sync::Arc;
use engine::support::logger::{configure, log, LoggerConfig};
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::window::api::cw::create_window;
use crate::d2::Demo2;
use crate::d2ec::create_engine_config;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let core = Arc::new(Demo2::new());
    let cfg = create_engine_config(core.clone());
    
    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(core, cfg).expect("window creation failed"); }
        Err(_e) => {
            log(LogLevel::Error, &|| "window creation failed".parse().unwrap());
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
