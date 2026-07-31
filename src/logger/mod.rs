use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Fatal = 4,
}

impl Level {
    pub fn name(&self) -> &str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Fatal => "FATAL",
        }
    }

    pub fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(Level::Debug),
            1 => Ok(Level::Info),
            2 => Ok(Level::Warn),
            3 => Ok(Level::Error),
            4 => Ok(Level::Fatal),
            _ => Ok(Level::Info),
        }
    }
}

#[allow(dead_code)]
pub fn format_log_entry(timestamp: u64, level: Level, msg: &str) -> String {
    let time = DateTime::<Utc>::from_timestamp_micros(timestamp as i64).expect("time format error");

    format!(
        "{} [{}] {}\n",
        time.format("%Y-%m-%d %H:%M:%S"),
        level.name(),
        msg
    )
}
