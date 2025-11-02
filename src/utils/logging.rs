//! A simple logging utility module.

use std::collections::HashMap;

/// Defines a 24-bit color with 8-bit rgb components.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

/// Struct for defining the logging format.
///
/// # Placeholders
/// - `{timestamp}`: Current timestamp
/// - `{timestampc}`: Current timestamp in color
/// - `{level}`: Log level
/// - `{levelc}`: Log level in color
/// - `{message}`: Log message
///
/// # Valid Formatting
/// - `*Text*`: Bold
/// - `_Text_`: Italic
/// - `~#RRGGBBText~`: Colored text (hex color code)
///
/// Note: ANSI codes will be ommitted in the output file.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LoggingFormat {
    pub template: String,
    pub timestamp_color: Color,
    /// Missing colors will fall back to default.
    pub level_colors: HashMap<LogLevel, Color>,
}

impl Default for LoggingFormat {
    fn default() -> Self {
        Self {
            template: String::from("[{timestampc}] [{levelc}] - {message}"),
            timestamp_color: Color {
                r: 100,
                g: 100,
                b: 100,
            },
            level_colors: {
                let mut map = HashMap::new();
                map.insert(LogLevel::Debug, Color { r: 0, g: 0, b: 255 });
                map.insert(LogLevel::Info, Color { r: 0, g: 255, b: 0 });
                map.insert(
                    LogLevel::Warn,
                    Color {
                        r: 255,
                        g: 255,
                        b: 0,
                    },
                );
                map.insert(LogLevel::Error, Color { r: 255, g: 0, b: 0 });
                map
            },
        }
    }
}

/// Enum for different logging levels.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct Logger {
    name: String,
    log_level: LogLevel,
    output_file: Option<String>,
    format: LoggingFormat,
}

impl Logger {
    fn new(
        name: String,
        log_level: Option<LogLevel>,
        output_file: Option<String>,
        format: Option<LoggingFormat>,
    ) -> Self {
        Self {
            name,
            log_level: log_level.unwrap_or_default(),
            output_file,
            format: format.unwrap_or_default(),
        }
    }

    pub fn builder(name: String) -> LoggerBuilder {
        LoggerBuilder::new(name)
    }

    pub fn debug(&self, message: &str) {
        todo!()
    }
    pub fn info(&self, message: &str) {
        todo!()
    }
    pub fn warn(&self, message: &str) {
        todo!()
    }
    pub fn error(&self, message: &str) {
        todo!()
    }

    fn log(&self, level: LogLevel, message: &str) {
        todo!()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(String::from("default"), None, None, None)
    }
}

pub struct LoggerBuilder {
    name: String,
    log_level: Option<LogLevel>,
    output_file: Option<String>,
    format: Option<LoggingFormat>,
}

impl LoggerBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            log_level: Default::default(),
            output_file: None,
            format: Default::default(),
        }
    }

    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    pub fn output_file(mut self, file: String) -> Self {
        self.output_file = Some(file);
        self
    }

    pub fn format(mut self, format: LoggingFormat) -> Self {
        self.format = Some(format);
        self
    }

    pub fn build(self) -> Logger {
        Logger::new(
            self.name,
            self.log_level,
            self.output_file,
            self.format,
        )
    }
}
