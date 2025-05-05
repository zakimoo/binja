use std::fmt::{self, Display};

use serde::{de, ser};

// A type alias for Result that uses the custom Error enum
pub type Result<T> = std::result::Result<T, Error>;

// Defining a custom Error enum to represent various error types
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    // A generic error message
    Message(String),

    // Error for invalid types with expected and found descriptions
    InvalidType {
        expected: String,
        found: String,
    },
    // Error for invalid values with expected and found descriptions
    InvalidValue {
        expected: String,
        found: String,
    },
    // Error for invalid lengths with expected and found descriptions
    InvalidLength {
        expected: String,
        found: String,
    },
    // Error for invalid variants with expected and found descriptions
    InvalidVariant {
        expected: String,
        found: String,
    },
    // Error for unknown fields with the field name and expected fields
    UnknownField {
        field: String,
        expected: Vec<String>,
    },
    // Error for missing fields with the field name
    MissingField {
        field: String,
    },
    // Error for duplicate fields with the field name
    DuplicateField {
        field: String,
    },

    LimitExceeded {
        limit: usize,
        size: usize,
    },

    NoEnoughData {
        expected: usize,
        available: usize,
    },

    InvalidBoolValue(u8),

    InvalidUtf8 {
        value: Vec<u8>,
    },
}

// Implementing the standard Error trait for the custom Error enum
impl std::error::Error for Error {}

// Implementing the serde::ser::Error trait for serialization errors
impl ser::Error for Error {
    // Converts a custom message into an Error::Message variant
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

// Implementing the serde::de::Error trait for deserialization errors
impl de::Error for Error {
    // Converts a custom message into an Error::Message variant
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            _ => formatter.write_str("Unknown error"),
        }
    }
}
