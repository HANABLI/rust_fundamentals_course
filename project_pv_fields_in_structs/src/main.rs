//! This module contains the configuration options for the application.
//! 


pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub enum LogOutput {
    Stdout,
    Stderr,
    File(String)
}
/// This struct contains the configuration options for controlling logging.
/// # Exemples:
/// ```
/// use project_pv_fields_in_structs::Logging;
/// let config = Logging::new();
/// ```
/// Create a new `Logging`struct with custom values:
/// ```
/// use project_pv_fields_in_structs::{Logging, LogLevel, LogOutput};
/// let config = Logging {
///     enabled: true,
///     level: LogLevel::Debug,
///     destination: LogOutput::File("Log.txt".to_string()),
/// };
/// ```
pub struct Logging {
    pub enable: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enable: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}

fn main() {
    let conf = Logging::new();
    println!("Logging enabled: {}", conf.enable);
}
