#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]// if windows, and release build, don't display console window

use std::sync::Arc;
use crate::d1ec::create_engine_config;
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::support::logger::{configure, log, LoggerConfig};
use engine::window::api::cw::create_window;
use crate::d1::Demo1;

pub mod d1wc;
mod d1ec;
mod d1m2d;
mod d1m3d;
mod d1kh;
mod d1mh;
mod d1cmd;
mod d1;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let core = Arc::new(Demo1::new());
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
