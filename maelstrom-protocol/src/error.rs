use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    /// Indicates that the requested operation could not be completed within a timeout.
    Timeout,
    /// Thrown when a client sends an RPC request to a node which does not exist.
    NodeNotFound,
    /// Use this error to indicate that a requested operation is not supported by the current implementation.
    NotSupported(Option<String>),
    /// Indicates that the operation definitely cannot be performed at this time--perhaps because
    /// the server is in a read-only state, has not yet been initialized, believes its peers to be down, and so on.
    TemporarilyUnavailable,
    /// The client's request did not conform to the server's expectations, and could not possibly have been processed.
    MalformedRequest,
    /// Indicates that some kind of general, indefinite error occurred.
    Crash,
    /// Indicates that some kind of general, definite error occurred. Use this as a catch-all for errors
    /// you can't otherwise categorize, when you specifically know that the requested operation has not taken place.
    Abort,
    /// The client requested an operation on a key which does not exist (assuming the operation should not automatically create missing keys).
    KeyDoesNotExist,
    /// The client requested the creation of a key which already exists, and the server will not overwrite it.
    KeyAlreadyExist,
    /// The requested operation expected some conditions to hold, and those conditions were not met.
    PreconditionFailed,
    /// The requested transaction has been aborted because of a conflict with another transaction.
    TxnConflict,
    /// Custom error that you can use. Composed of a code and an String error
    /// codes 1000 and above are free for your own purposes.
    CustomError((i32, Option<String>)),
}

impl Error {
    /// Returns the numeric error code associated with this error.
    pub fn code(&self) -> i32 {
        match self {
            Self::Timeout => 0,
            Self::NodeNotFound => 1,
            Self::NotSupported(_) => 10,
            Self::TemporarilyUnavailable => 11,
            Self::MalformedRequest => 12,
            Self::Crash => 13,
            Self::Abort => 14,
            Self::KeyDoesNotExist => 20,
            Self::KeyAlreadyExist => 21,
            Self::PreconditionFailed => 22,
            Self::TxnConflict => 30,
            Self::CustomError((code, _)) => *code,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::NodeNotFound => write!(f, "NodeNotFound"),
            Self::NotSupported(err) => {
                if let Some(err) = err {
                    write!(f, "NotSupported: {}", err)
                } else {
                    write!(f, "NotSupported")
                }
            }
            Self::TemporarilyUnavailable => write!(f, "TemporarilyUnavailable"),
            Self::MalformedRequest => write!(f, "MalformedRequest"),
            Self::Crash => write!(f, "Crash"),
            Self::Abort => write!(f, "Abort"),
            Self::KeyDoesNotExist => write!(f, "KeyDoesNotExist"),
            Self::KeyAlreadyExist => write!(f, "KeyAlreadyExist"),
            Self::PreconditionFailed => write!(f, "PreconditionFailed"),
            Self::TxnConflict => write!(f, "TxnConflict"),
            Self::CustomError((_, err)) => {
                if let Some(err) = err {
                    write!(f, "CustomError: {}", err)
                } else {
                    write!(f, "CustomError")
                }
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::CustomError((1003, Some(e.to_string())))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::CustomError((1004, Some(e.to_string())))
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_error_serialization() {
        let err = Error::NotSupported(Some("unsupported operation".to_string()));
        let expected = json!({"NotSupported": "unsupported operation"});
        let actual = serde_json::to_value(err).unwrap();
        assert_eq!(actual, expected);
    }
}
