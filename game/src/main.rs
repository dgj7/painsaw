use logger::log;
use logger::model::log_config::{LogConfigPair, LoggingConfig};
use logger::model::log_level::LogLevel;
use logger::model::log_target::LogTarget;
use window::create_window;
use window::model::window_request::{WindowDimensions, WindowRequest};

fn main() {
    let lc = LoggingConfig::new(&*vec!(LogConfigPair { level: LogLevel::Debug, target: LogTarget::StdOut }));
    log(&lc, LogLevel::Info, &|| "begin");

    let wr = WindowRequest::new(WindowDimensions::Fullscreen, "painsaw");
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
