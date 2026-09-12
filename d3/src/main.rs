#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // if windows, and release build, don't display console window

mod d3;
mod d3ec;
mod d3kh;
mod d3mh;
mod d3wc;

use engine::support::logger::{configure, log, LoggerConfig};
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::window::api::cw::create_window;
use engine::window::Window;
use crate::d3::Demo3;
use crate::d3ec::create_engine_config;

fn main() {
    configure(LoggerConfig { level: LogLevel::Debug, target: LogTarget::StdOut, });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let mut game = Demo3::new();
    let cfg = create_engine_config();

    match create_window(&cfg) {
        Ok(mut win) => { win.begin_event_handling(&mut game, cfg.clone()).expect("window creation failed"); }
        Err(_e) => { log(LogLevel::Error, &|| { "window creation failed".parse().unwrap() });std::process::exit(1); }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
