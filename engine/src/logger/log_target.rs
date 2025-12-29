use crate::logger::log_level::LogLevel;
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use std::panic::Location;
use std::path::Path;

#[derive(Debug, Clone)]
pub enum LogTarget {
    File { path: String },
    EngineConsole,
    StdOut,
}

const MAX: usize = 80;
const LEVEL: usize = 6;
const CALLER: usize = 30;
const TIME: usize = 9;
const MSG: usize = MAX - (LEVEL + CALLER + TIME);

impl LogTarget {
    pub(crate) fn print(
        &self,
        level: &LogLevel,
        when: DateTime<Local>,
        caller: &Location,
        message: &String,
    ) {
        let filename = Path::new(caller.file())
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let line = caller.line();

        match self {
            LogTarget::File { path: _ } => panic!("not implemented!"),
            LogTarget::EngineConsole => panic!("not implemented!"),
            LogTarget::StdOut => println!(
                "{:LEVEL$}{:TIME$}{:CALLER$}: {}",
                colorize_lvl(level),
                when.format("%H:%M:%S").to_string().as_str().blue(),
                truncate(format!("{}:{}", filename, line).as_str(), CALLER).green(),
                colorize_msg(level, message),
            ),
        }
    }
}

fn truncate(string: &str, max: usize) -> &str {
    let len = string.len();
    if len > max {
        string
            .char_indices()
            .rev()
            .nth(max - 1)
            .map(|(i, _)| &string[i..])
            .unwrap_or(string)
    } else {
        string
    }
}

const IC: u8 = 150;
const DC: u8 = 110;
const TC: u8 = 70;

fn colorize_lvl(level: &LogLevel) -> ColoredString {
    match level {
        LogLevel::Error => truncate("ERROR", LEVEL).red(),
        LogLevel::Warning => truncate("WARN", LEVEL).yellow().bold(),
        LogLevel::Info => truncate("INFO", LEVEL).truecolor(IC, IC, IC).bold(),
        LogLevel::Debug => truncate("DEBUG", LEVEL).truecolor(DC, DC, DC).dimmed(),
        LogLevel::Trace => truncate("TRACE", LEVEL).truecolor(TC, TC, TC).dimmed(),
    }
}

// todo: don't truncate messages
fn colorize_msg(level: &LogLevel, msg: &String) -> ColoredString {
    match level {
        LogLevel::Error => truncate(msg, MSG).white().bold(),
        LogLevel::Warning => truncate(msg, MSG).white(),
        LogLevel::Info => truncate(msg, MSG).truecolor(IC, IC, IC).bold(),
        LogLevel::Debug => truncate(msg, MSG).truecolor(DC, DC, DC).dimmed(),
        LogLevel::Trace => truncate(msg, MSG).truecolor(TC, TC, TC).dimmed(),
    }
}
