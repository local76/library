//! Custom first-class error types for library APIs.
//!
//! Provides structured error classification to prevent panics and raw String error passing.

use std::fmt;
use std::error::Error;
use std::io;
use std::path::PathBuf;

/// The primary error type for all operations in the library library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryError {
    /// Filesystem / generic I/O failure. Use this for std::fs, std::net,
    /// OpenOptions, read_to_string, etc.
    Io(String),
    /// Errors occurring during local IPC named pipe or socket operations ONLY.
    Ipc(String),
    /// Errors occurring during command-line argument parsing.
    Cli(String),
    /// Errors occurring during background service controls (start/stop/restart).
    Service(String),
    /// Errors occurring during lifecycle single instance lock acquisitions.
    Guard(String),
    /// Errors occurring during OpenRGB communication or protocol parsing.
    Rgb(String),
    /// Errors occurring during platform display/system queries.
    Platform(String),
    /// Errors occurring during text formatting or date/uptime calculations.
    Formatting(String),
    /// Configuration parse / write / path resolution failure.
    Config(String),
    /// Path did not exist where one was required.
    NotFound { kind: &'static str, path: PathBuf },
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg)       => write!(f, "I/O error: {}", msg),
            Self::Ipc(msg)      => write!(f, "IPC error: {}", msg),
            Self::Cli(msg)      => write!(f, "CLI error: {}", msg),
            Self::Service(msg)  => write!(f, "Service error: {}", msg),
            Self::Guard(msg)    => write!(f, "Lifecycle guard error: {}", msg),
            Self::Rgb(msg)      => write!(f, "RGB protocol error: {}", msg),
            Self::Platform(msg) => write!(f, "Platform query error: {}", msg),
            Self::Formatting(m) => write!(f, "Formatting error: {}", m),
            Self::Config(msg)   => write!(f, "Config error: {}", msg),
            Self::NotFound { kind, path } => write!(f, "{} not found: {}", kind, path.display()),
        }
    }
}

impl Error for LibraryError {}

impl LibraryError {
    /// True only for genuine pipe/socket termination. File/registry/network
    /// errors are now NEVER classified as IPC termination.
    pub fn is_ipc_termination(&self) -> bool {
        match self {
            Self::Ipc(msg) => {
                let lower = msg.to_lowercase();
                lower.contains("aborted")
                    || lower.contains("invalid")
                    || lower.contains("broken pipe")
                    || lower.contains("connection reset")
                    || lower.contains("pipe is being closed")
            }
            _ => false,
        }
    }
}

/// A specialized `Result` type alias utilizing LibraryError.
pub type Result<T> = std::result::Result<T, LibraryError>;

/// Convenient Result type alias utilizing LibraryError.
pub type LibraryResult<T> = Result<T>;

// Explicit, narrow From — not a blanket conversion. Call-sites that genuinely
// are doing pipe I/O can use .map_err(|e| LibraryError::Ipc(e.to_string())).
impl From<io::Error> for LibraryError {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
