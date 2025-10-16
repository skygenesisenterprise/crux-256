// Custom error types

#[derive(Debug)]
pub enum CruxError {
    InvalidKeyLength,
    InvalidBlockLength,
    InvalidPadding,
    InvalidTag,
    DecryptionFailed,
}

pub type Result<T> = std::result::Result<T, CruxError>;

impl std::fmt::Display for CruxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CruxError::InvalidKeyLength => write!(f, "Invalid key length"),
            CruxError::InvalidBlockLength => write!(f, "Invalid block length"),
            CruxError::InvalidPadding => write!(f, "Invalid padding"),
            CruxError::InvalidTag => write!(f, "Invalid authentication tag"),
            CruxError::DecryptionFailed => write!(f, "Decryption failed"),
        }
    }
}

impl std::error::Error for CruxError {}