#[derive(Debug)]
pub enum ErrorKind {
    NothingToRead,
    MessageSizeExceeded,
    Io { source: std::io::Error },
    ParsingJson { source: serde_json::Error },
    DeserializingBson { source: bson::de::Error },
    SerializingBson { source: bson::ser::Error },
    ConversionBson { source: bson::document::ValueAccessError },
    MalformedMessage { message: String },
    PoisonedLock { message: String },
    SendError { message: String },
    SystemTime { source: std::time::SystemTimeError },
    Configuration { message: String },
    Uuid { message: String },
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::NothingToRead => {
                write!(formatter, "Unable to parse a message, closing the connection")
            }
            ErrorKind::MessageSizeExceeded => {
                write!(formatter, "Message maximum size exceeded")
            }
            ErrorKind::Io { source } => {
                write!(formatter, "Io > {}", source)
            }
            ErrorKind::ParsingJson { source } => {
                write!(formatter, "Parsing JSON > {}", source)
            }
            ErrorKind::DeserializingBson { source } => {
                write!(formatter, "Deserializing BSON > {}", source)
            }
            ErrorKind::SerializingBson { source } => {
                write!(formatter, "Serializing BSON > {}", source)
            }
            ErrorKind::ConversionBson { source } => {
                write!(formatter, "Conversion BSON > {}", source)
            }
            ErrorKind::MalformedMessage { message } => {
                write!(formatter, "Received a message with incorrect format > {}", message)
            }
            ErrorKind::PoisonedLock { message } => {
                write!(formatter, "Ran into a poisoned lock > {}", message)
            }
            ErrorKind::SendError { message } => {
                write!(formatter, "Channel sending > {}", message)
            }
            ErrorKind::SystemTime { source } => {
                write!(formatter, "System time > {}", source)
            }
            ErrorKind::Configuration { message } => {
                write!(formatter, "Incorrect configuration > {}", message)
            }
            ErrorKind::Uuid { message } => {
                write!(formatter, "Uuid error > {}", message)
            }
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Shared Code Error > {}", self.kind)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error { kind: kind }
    }
}

impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        // Unwrap embedded io::Error
        match source.classify() {
            serde_json::error::Category::Io => {
                std::io::Error::from(source).into()
            }
            _ => Error {
                kind: ErrorKind::ParsingJson {
                    source: source,
                }
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::Io {
                source: source,
            }
        }
    }
}

impl From<ErrorKind> for std::io::Error {
    fn from(kind: ErrorKind) -> Self {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            Error { kind: kind }
        )
    }
}

impl From<bson::de::Error> for Error {
    fn from(source: bson::de::Error) -> Self {
        // Unwrap embedded io::Error
        match &source {
            bson::de::Error::Io(io_error) => match io_error.kind() {
                std::io::ErrorKind::InvalidData => Error {
                    kind: ErrorKind::Io {
                        source: std::io::ErrorKind::InvalidData.into()
                    }
                },
                _ => Error {
                    kind: ErrorKind::DeserializingBson {
                        source: source,
                    }
                }
            }
            _ => Error {
                kind: ErrorKind::DeserializingBson {
                    source: source,
                }
            }
        }
    }
}

impl From<bson::ser::Error> for Error {
    fn from(source: bson::ser::Error) -> Self {
        // Unwrap embedded io::Error
        match &source {
            bson::ser::Error::Io(io_error) => match io_error.kind() {
                std::io::ErrorKind::InvalidData => Error {
                    kind: ErrorKind::Io {
                        source: std::io::ErrorKind::InvalidData.into()
                    }
                },
                _ => Error {
                    kind: ErrorKind::SerializingBson {
                        source: source,
                    }
                }
            }
            _ => Error {
                kind: ErrorKind::SerializingBson {
                    source: source,
                }
            }
        }
    }
}

impl From<bson::document::ValueAccessError> for Error {
    fn from(source: bson::document::ValueAccessError) -> Self {
        Error {
            kind: ErrorKind::ConversionBson {
                source: source,
            }
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(source: std::sync::PoisonError<T>) -> Self {
        Error {
            kind: ErrorKind::PoisonedLock {
                message: format!("{}", source),
            }
        }
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for Error {
    fn from(source: std::sync::mpsc::SendError<T>) -> Self {
        Error {
            kind: ErrorKind::SendError {
                message: format!("{}", source)
            }
        }
    }
}

impl From<std::time::SystemTimeError> for Error {
    fn from(source: std::time::SystemTimeError) -> Self {
        Error {
            kind: ErrorKind::SystemTime {
                source: source,
            }
        }
    }
}

impl From<uuid::Error> for Error {
    fn from(source: uuid::Error) -> Self {
        Error {
            kind: ErrorKind::Uuid {
                message: format!("{}", source),
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn with_error_report<F: FnOnce() -> Result<()>>(run: F) {
    let result = run();

    match &result {
        Err(error) => {
            println!("Error > {}", error);
        }
        _ => {}
    };
}

pub fn is_would_block_io_error(error: &std::io::Error) -> bool {
    match error.kind() {
        std::io::ErrorKind::WouldBlock => true,
        _ => false
    }
}

pub fn is_would_block_error(error: &Error) -> bool {
    match &error.kind {
        ErrorKind::Io { source } => is_would_block_io_error(source),
        _ => false
    }
}

pub fn is_would_block_result<T>(result: &Result<T>) -> bool {
    match result {
        Err(error) => is_would_block_error(error),
        _ => false
    }
}

pub fn is_would_block_io_result<T>(result: &std::io::Result<T>) -> bool {
    match result {
        Err(error) => is_would_block_io_error(error),
        _ => false
    }
}

impl<T> From<ErrorKind> for Result<T> {
    fn from(kind: ErrorKind) -> Self {
        Err(kind.into())
    }
}
