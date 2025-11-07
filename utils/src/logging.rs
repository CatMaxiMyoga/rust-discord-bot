//! A simple logging utility module.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

static DEFAULT_TIMEZONE: chrono_tz::Tz = chrono_tz::Europe::Berlin;

lazy_static! {
    static ref COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
}

/// Defines a 24-bit color with 8-bit rgb components.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Create a Color from 8-bit rgb components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }

    /// Create a Color from a hex string (e.g., "#RRGGBB" or "RRGGBB").
    /// Accepts both 3-digit and 6-digit hex codes.
    pub fn hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Self::rgb(r, g, b))
        } else {
            None
        }
    }

    /// Generate and return the ANSI escape code for this color.
    pub fn ansi_code(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.red, self.green, self.blue)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(255, 255, 255)
    }
}

/// Struct for defining the logging format.
///
/// # Placeholders
/// - `{{timestamp}}`: Current timestamp
/// - `{{timestampc}}`: Current timestamp in color
/// - `{{level}}`: Log level
/// - `{{levelc}}`: Log level in color
/// - `{{message}}`: Log message
/// - `{{messagec}}`: Log message in color
///
/// # Valid Formatting
/// - `*Text*`: Bold
/// - `_Text_`: Italic
/// - `~#RRGGBBText~`: Colored text (hex color code)
///
/// # Notes
/// - ANSI codes will be omitted in the output file.
/// - Template string will not be validated.
/// - You cannot nest color formatting.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LoggingFormat {
    /// Template string for log messages. For details, see the [`LoggingFormat`] documentation.
    pub template: String,
    /// Color for the timestamp when using `{{timestampc}}`.
    pub timestamp_color: Color,
    /// Format for the timestamp. Uses `chrono` formatting.
    pub timestamp_format: String,
    /// Timezone for the timestamp.
    pub timezone: chrono_tz::Tz,
    /// Colors for different log levels when using `{{levelc}}`. Missing levels default to
    /// [`Default::default()`].
    pub level_colors: HashMap<LogLevel, Color>,
    /// Colors for different log levels when using `{{messagec}}`. Missing levels default to
    /// [`Default::default()`].
    pub message_colors: HashMap<LogLevel, Color>,
}

impl Default for LoggingFormat {
    fn default() -> Self {
        let mut default_colors: HashMap<LogLevel, Color> = HashMap::new();
        default_colors.insert(LogLevel::Debug, Color::rgb(0, 255, 255));
        default_colors.insert(LogLevel::Info, Color::rgb(0, 255, 0));
        default_colors.insert(LogLevel::Warn, Color::rgb(255, 255, 0));
        default_colors.insert(LogLevel::Error, Color::rgb(255, 0, 0));

        Self {
            template: String::from("*[{{timestampc}}] [{{levelc}}]* {{messagec}}"),
            timestamp_color: Color::rgb(100, 100, 100),
            timestamp_format: String::from("%Y-%m-%d %H:%M:%S %Z"),
            timezone: DEFAULT_TIMEZONE,
            level_colors: default_colors.clone(),
            message_colors: default_colors.clone(),
        }
    }
}

/// Enum for different logging levels.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum LogLevel {
    /// Shows debug log messages
    Debug,
    #[default]
    /// Shows info log messages
    Info,
    /// Shows warning log messages
    Warn,
    /// Shows error log messages
    Error,
    /// Turns off logging
    Off,
}

/// A simple logger with configurable log levels, output file, and formatting.
#[derive(Debug, Clone)]
pub struct Logger {
    log_level: LogLevel,
    output_file: Option<String>,
    format: LoggingFormat,
}

impl Logger {
    fn new(
        log_level: Option<LogLevel>,
        output_file: Option<String>,
        format: Option<LoggingFormat>,
    ) -> Self {
        Self {
            log_level: log_level.unwrap_or_default(),
            output_file,
            format: format.unwrap_or_default(),
        }
    }

    /// Create a new builder for constructing a Logger.
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::new()
    }

    /// Log a debug message. Only logs if the log level is set to Debug.
    pub fn debug(&self, message: &str) {
        if self.log_level <= LogLevel::Debug {
            self.log(LogLevel::Debug, message);
        }
    }

    /// Log an info message. Only logs if the log level is set to Info or lower.
    pub fn info(&self, message: &str) {
        if self.log_level <= LogLevel::Info {
            self.log(LogLevel::Info, message);
        }
    }

    /// Log a warning message. Only logs if the log level is set to Warn or lower.
    pub fn warn(&self, message: &str) {
        if self.log_level <= LogLevel::Warn {
            self.log(LogLevel::Warn, message);
        }
    }

    /// Log an error message. Only logs if the log level is set to Error or lower.
    pub fn error(&self, message: &str) {
        if self.log_level <= LogLevel::Error {
            self.log(LogLevel::Error, message);
        }
    }

    /// Set the log level for the logger.
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// Set the output file for the logger. Use `None` to disable file logging.
    pub fn set_output_file(&mut self, file: Option<String>) {
        self.output_file = file;
    }

    fn log(&self, level: LogLevel, message: &str) {
        let template = &self.format.template;
        let chars: Vec<char> = template.chars().collect();
        let chars_len = chars.len();

        let mut term_output = String::new();
        let mut file_output = String::new();
        let mut bold = false;
        let mut italic = false;
        let mut colored = String::new();

        let mut i = 0;
        while i < chars_len {
            let c = chars[i];
            match c {
                '{' => {
                    if !(i + 1 < chars_len && chars[i + 1] == '{') {
                        term_output.push(c);
                        file_output.push(c);
                        i += 1;
                        continue;
                    }

                    let j = chars[i..].iter().position(|&x| x == '}');
                    if j.is_none() {
                        term_output.push(c);
                        file_output.push(c);
                        i += 1;
                        continue;
                    }

                    let j = j.unwrap() + i;
                    if !(j + 1 < chars_len && chars[j + 1] == '}') {
                        term_output.push(c);
                        file_output.push(c);
                        i += 1;
                        continue;
                    }

                    let placeholder_string = chars[i + 2..j].iter().collect::<String>();
                    let placeholder = placeholder_string.as_str();

                    match placeholder {
                        "timestamp" | "timestampc" => {
                            let timestamp = chrono::Utc::now()
                                .with_timezone(&self.format.timezone)
                                .format(&self.format.timestamp_format)
                                .to_string();
                            file_output.push_str(timestamp.as_str());
                            if placeholder == "timestampc" {
                                let color_code = self.format.timestamp_color.ansi_code();
                                term_output.push_str(color_code.as_str());
                                term_output.push_str(timestamp.as_str());
                                term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                            } else {
                                term_output.push_str(timestamp.as_str());
                            }
                            i = j + 2;
                            continue;
                        }
                        "level" | "levelc" => {
                            let level_str = match level {
                                LogLevel::Debug => "DEBUG",
                                LogLevel::Info => "INFO",
                                LogLevel::Warn => "WARN",
                                LogLevel::Error => "ERROR",
                                _ => "",
                            };
                            file_output.push_str(level_str);
                            if placeholder == "levelc" {
                                let color = self.format.level_colors.get(&level);
                                let color_code = color.unwrap_or(&Color::default()).ansi_code();
                                term_output.push_str(color_code.as_str());
                                term_output.push_str(level_str);
                                term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                            } else {
                                term_output.push_str(level_str);
                            }
                            i = j + 2;
                            continue;
                        }
                        "message" | "messagec" => {
                            file_output.push_str(message);
                            if placeholder == "messagec" {
                                let color = self.format.message_colors.get(&level);
                                let color_code = color.unwrap_or(&Color::default()).ansi_code();
                                term_output.push_str(color_code.as_str());
                                term_output.push_str(message);
                                term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                            } else {
                                term_output.push_str(message);
                            }
                            i = j + 2;
                            continue;
                        }
                        _ => {
                            term_output.push(c);
                            file_output.push(c);
                        }
                    }
                }
                '*' => {
                    bold = !bold;
                    term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                }
                '_' => {
                    italic = !italic;
                    term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                }
                '~' => {
                    if !colored.is_empty() {
                        // Closing color tag
                        colored.clear();
                        term_output.push_str(ansi_reset(bold, italic, &colored).as_str());
                        i += 1;
                        continue;
                    }

                    if i + 8 >= chars_len {
                        term_output.push(c);
                        file_output.push(c);
                        i += 1;
                        continue;
                    }
                    let hex_code: String = chars[i + 1..i + 8].iter().collect();

                    if !COLOR_REGEX.is_match(&hex_code) {
                        term_output.push(c);
                        file_output.push(c);
                        i += 1;
                        continue;
                    }

                    // Unwrap is safe due to regex check
                    let color = Color::hex(&hex_code).unwrap();

                    colored = color.ansi_code();
                    term_output.push_str(colored.as_str());
                    i += 8;
                    continue;
                }
                _ => {
                    term_output.push(c);
                    file_output.push(c);
                }
            };
            i += 1;
        }

        term_output.push_str(ansi_reset(false, false, "").as_str());
        println!("{}", term_output);

        if let Some(file) = &self.output_file {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file)
                .unwrap();
            writeln!(file, "{}", file_output).unwrap();
        }

        fn ansi_reset(bold: bool, italic: bool, colored: &str) -> String {
            let mut output = String::from("\x1b[0m");
            if bold {
                output.push_str("\x1b[1m");
            }
            if italic {
                output.push_str("\x1b[3m");
            }
            if !colored.is_empty() {
                output.push_str(colored);
            }
            output
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

/// Builder for constructing a Logger with custom settings.
#[derive(Debug, Clone, Default)]
pub struct LoggerBuilder {
    log_level: Option<LogLevel>,
    output_file: Option<String>,
    format: Option<LoggingFormat>,
}

impl LoggerBuilder {
    /// Create a new LoggerBuilder with default settings.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the log level for the logger.
    pub fn log_level(mut self, level: LogLevel) -> Self {
        self.log_level = Some(level);
        self
    }

    /// Set the output file for the logger.
    pub fn output_file(mut self, file: String) -> Self {
        self.output_file = Some(file);
        self
    }

    /// Set the logging format for the logger.
    pub fn format(mut self, format: LoggingFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Build and return the Logger with the specified settings.
    pub fn build(self) -> Logger {
        Logger::new(self.log_level, self.output_file, self.format)
    }
}
