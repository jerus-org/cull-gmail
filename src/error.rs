use thiserror::Error;

/// Error messages for cull-gmail
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration directory not set
    #[error("Configuration directory not set")]
    DirectoryUnset,
    /// Expansion of home directory in `{0}` failed
    #[error("Expansion of home directory in `{0}` failed")]
    HomeExpansionFailed(String),
    /// Directory creation failed for `{0}`
    #[error("Directory creation failed for `{0:?}`")]
    DirectoryCreationFailed((String, Box<std::io::Error>)),
    /// Error from the google_gmail1 crate
    // #[error("Google Gmail1 says: {0}")]
    #[error(transparent)]
    GoogleGmail1(#[from] Box<google_gmail1::Error>),
    // /// Error from the google_clis_common crate
    // #[error("Google CLIs Common says: {0}")]
    // InvalidOptionsError(google_clis_common::CLIError, i16),
    // /// Other error
    // #[error("Error reported: {0}")]
    // Other(#[from] String),
}
