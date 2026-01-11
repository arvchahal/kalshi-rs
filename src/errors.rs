use std::fmt;
#[derive(Debug)]
pub enum KalshiError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    IoError(std::io::Error),
    Other(String),
}
impl fmt::Display for KalshiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KalshiError::RequestError(e) => write!(f, "Request error: {}", e),
            KalshiError::ParseError(e) => write!(f, "Parse error: {}", e),
            KalshiError::IoError(e) => write!(f, "IO error: {}", e),
            KalshiError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}
impl std::error::Error for KalshiError {}
impl From<reqwest::Error> for KalshiError {
    fn from(err: reqwest::Error) -> Self {
        KalshiError::RequestError(err)
    }
}
impl From<serde_json::Error> for KalshiError {
    fn from(err: serde_json::Error) -> Self {
        KalshiError::ParseError(err)
    }
}
impl From<std::io::Error> for KalshiError {
    fn from(err: std::io::Error) -> Self {
        KalshiError::IoError(err)
    }
}
impl From<String> for KalshiError {
    fn from(s: String) -> KalshiError{
        KalshiError::Other(s)
    }
}
