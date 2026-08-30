#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)] // if windows, and release build, don't display console window

use crate::d1::Demo1;
use crate::d1ec::create_engine_config;
use engine::support::logger::log_level::LogLevel;
use engine::support::logger::log_target::LogTarget;
use engine::support::logger::{configure, log, LoggerConfig};
use engine::window::api::cw::create_window;
use engine::window::Window;

mod d1;
mod d1cmd;
mod d1ec;
mod d1kh;
mod d1m2d;
mod d1m3d;
mod d1mh;
pub mod d1wc;

fn main() {
    configure(LoggerConfig {
        level: LogLevel::Debug,
        target: LogTarget::StdOut,
    });
    log(LogLevel::Info, &|| "main(): begin".parse().unwrap());

    let game = Demo1::new();
    let cfg = create_engine_config();

    match create_window(&cfg) {
        Ok(mut win) => {
            win.begin_event_handling(cfg.clone(), &game)
                .expect("window creation failed");
        }
        Err(_e) => {
            log(LogLevel::Error, &|| {
                "window creation failed".parse().unwrap()
            });
            std::process::exit(1);
        }
    }

    log(LogLevel::Info, &|| "main(): end.".parse().unwrap());
}
