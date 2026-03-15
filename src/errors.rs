use std::fmt::Display;

/// HTTP return codes
pub enum ReturnCodes {
    Success,
    BackendError,
    NotFound,
}

impl Display for ReturnCodes {
    /// matches the HTTP convention
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ReturnCodes::Success => "200",
            ReturnCodes::BackendError => "500",
            ReturnCodes::NotFound => "404",
        };
        write!(f, "{}", str)
    }
}
