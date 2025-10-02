use thiserror::Error;

/// Error messages for cull-gmail
#[derive(Debug, Error)]
pub enum Error {
    /// Error from the google_gmail1 crate
    #[error("Google Gmail1 says: {0}")]
    GoogleGmail1(#[from] google_gmail1::Error),
    /// Error from the google_clis_common crate
    #[error("Google CLIs Common says: {0}")]
    InvalidOptionsError(google_clis_common::CLIError, i16),
}
