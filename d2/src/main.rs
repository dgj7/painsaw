#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // if windows, and release build, don't display console window

use engine::window::Window;
mod d2;
mod d2ec;
mod d2kh;
mod d2m2d;
mod d2mh;
mod d2wc;

use crate::d2::Demo2;
use crate::d2ec::create_engine_config;
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::support::logger::{configure, log, LoggerConfig};
use engine::window::api::cw::create_window;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut, });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let mut game = Demo2::new();
    let cfg = create_engine_config();

    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(&mut game, cfg.clone()).expect("window creation failed"); }
        Err(_e) => { log(LogLevel::Error, &|| { "window creation failed".parse().unwrap() });std::process::exit(1); }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
