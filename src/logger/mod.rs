use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum Level {
    DEBUG = 0,
    INFO = 1,
    WARN = 2,
    ERROR = 3,
    FATAL = 4,
}

impl Level {
    pub fn name(&self) -> &str {
        match self {
            Level::DEBUG => "DEBUG",
            Level::INFO => "INFO",
            Level::WARN => "WARN",
            Level::ERROR => "ERROR",
            Level::FATAL => "FATAL",
        }
    }

    pub fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(Level::DEBUG),
            1 => Ok(Level::INFO),
            2 => Ok(Level::WARN),
            3 => Ok(Level::ERROR),
            4 => Ok(Level::FATAL),
            _ => Ok(Level::INFO),
        }
    }
}

#[allow(dead_code)]
pub fn format_log_entry(timestamp: u64, level: Level, msg: &str) -> String {
    let time = DateTime::<Utc>::from_timestamp_micros(timestamp as i64)
        .expect("time format error");
    
    format!(
        "{} [{}] {}\n",
        time.format("%Y-%m-%d %H:%M:%S"),
        level.name(),
        msg
    )
} 