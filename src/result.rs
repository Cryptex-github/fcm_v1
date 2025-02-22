/// Custom result type for FCM API calls.
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for FCM API calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// OAuth2 authentication error.
    Auth,
    /// Arbitrary configuration error (e.g. unable to initialize TLS backend).
    Config,
    /// Deserialization error (i.e. unexpected result format received from server).
    Deserialization,
    /// FCM server error (returned directly to caller).
    FCM { status_code: u16, body: String },
    /// Timed out while waiting for server. According to Google, [the server should use exponential back-off to
    /// deal with timeout errors](https://firebase.google.com/docs/cloud-messaging/server).
    Timeout,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Auth => write!(f, "authentication error"),
            Error::Config => write!(f, "configuration error"),
            Error::Deserialization => write!(f, "deserialization error"),
            Error::FCM { status_code, body } => write!(f, "firebase error: received {status_code} status code with body: {body}"),
            Error::Timeout => write!(f, "timeout"),
        }
    }
}

impl std::error::Error for Error {}
