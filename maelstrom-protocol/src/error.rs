#[derive(Debug, Clone)]
pub enum MaelstromError {
    /// Indicates that the requested operation could not be completed within a timeout.
    Timeout = 0,
    /// Thrown when a client sends an RPC request to a node which does not exist.
    NodeNotFound = 1,
    /// Use this error to indicate that a requested operation is not supported by the current implementation.
    NotSupported = 10,
    /// Indicates that the operation definitely cannot be performed at this time--perhaps because
    /// the server is in a read-only state, has not yet been initialized, believes its peers to be down, and so on.
    TemporarilyUnavailable = 11,
    /// The client's request did not conform to the server's expectations, and could not possibly have been processed.
    MalformedRequest = 12,
    /// Indicates that some kind of general, indefinite error occurred.
    Crash = 13,
    /// Indicates that some kind of general, definite error occurred. Use this as a catch-all for errors
    /// you can't otherwise categorize, when you specifically know that the requested operation has not taken place.
    Abort = 14,
    /// The client requested an operation on a key which does not exist (assuming the operation should not automatically create missing keys).
    KeyDoesNotExist = 20,
    /// The client requested the creation of a key which already exists, and the server will not overwrite it.
    KeyAlreadyExist = 21,
    /// The requested operation expected some conditions to hold, and those conditions were not met.
    PreconditionFailed = 22,
    /// The requested transaction has been aborted because of a conflict with another transaction.
    TxnConflict = 30,
    /// Custom error that you can use. Composed of a code and an String error
    /// codes 10000 and above are free for your own purposes.
    CustomError((u64, String)),
}

impl MaelstromError {
    /// Returns the numeric error code associated with this error.
    pub fn code(&self) -> u64 {
        match self {
            Self::Timeout => 0,
            Self::NodeNotFound => 1,
            Self::NotSupported => 10,
            Self::TemporarilyUnavailable => 11,
            Self::MalformedRequest => 12,
            Self::Crash => 13,
            Self::Abort => 14,
            Self::KeyDoesNotExist => 20,
            Self::KeyAlreadyExist => 21,
            Self::PreconditionFailed => 22,
            Self::TxnConflict => 30,
            Self::CustomError((code, _)) => {
                if *code > 1_000 {
                    *code
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl std::fmt::Display for MaelstromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::NodeNotFound => write!(f, "NodeNotFound"),
            Self::NotSupported => write!(f, "NotSupported"),
            Self::TemporarilyUnavailable => write!(f, "TemporarilyUnavailable"),
            Self::MalformedRequest => write!(f, "MalformedRequest"),
            Self::Crash => write!(f, "Crash"),
            Self::Abort => write!(f, "Abort"),
            Self::KeyDoesNotExist => write!(f, "KeyDoesNotExist"),
            Self::KeyAlreadyExist => write!(f, "KeyAlreadyExist"),
            Self::PreconditionFailed => write!(f, "PreconditionFailed"),
            Self::TxnConflict => write!(f, "TxnConflict"),
            Self::CustomError((_, err)) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for MaelstromError {}
