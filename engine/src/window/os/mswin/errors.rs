use crate::support::logger::log_caller;
use crate::support::logger::log_level::LogLevel;
use std::io::Error;
use std::panic::Location;

///
/// Check the last message for Windows.
///
/// I don't know why last_os_error doesn't expose the code and message more clearly.
///
#[track_caller]
pub fn check_errors_mswin(caller: &str) {
    let loe = Error::last_os_error();
    let loe_message = loe.to_string();
    match loe.raw_os_error() {
        None => {}
        Some(raw_error_code) => {
            if raw_error_code > 0 {
                log_caller(LogLevel::Error, Location::caller(), &|| String::from(format!("{}: {}: {}", caller, raw_error_code, loe_message)));
            }
        }
    }
}
