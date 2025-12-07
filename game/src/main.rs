use logger::log;
use logger::model::log_config::{LogConfigPair, LoggerConfig};
use logger::model::log_level::LogLevel;
use logger::model::log_target::LogTarget;
use window::create_window;
use window::model::window_config::{WindowDimensions, WindowConfig};

fn main() {
    let lc = LoggerConfig::new(&*vec!(LogConfigPair { level: LogLevel::Debug, target: LogTarget::StdOut }));
    log(&lc, LogLevel::Info, &|| "begin");

    let wr = WindowConfig::new(WindowDimensions::Fullscreen, "painsaw");
    match create_window(&wr) {
        Ok(win) => {
            win.begin_display();
        },
        Err(_e) => {
            // todo: logger here
        }
    }

    log(&lc, LogLevel::Info, &|| "end.");
}
