use thiserror::Error;

/// Error messages for cull-gmail
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid paging mode option
    #[error("Invalid paging mode option")]
    InvalidPagingMode,
    /// Configuration directory not set
    #[error("Configuration directory not set")]
    DirectoryUnset,
    /// Expansion of home directory in `{0}` failed
    #[error("Expansion of home directory in `{0}` failed")]
    HomeExpansionFailed(String),
    /// No rule selector specified (i.e. --id or --label)
    #[error("No rule selector specified (i.e. --id or --label)")]
    NoRuleSelector,
    /// No rule for label
    #[error("No rule for label {0}")]
    NoRuleFoundForLabel(String),
    /// Label not found in the rule set
    #[error("Label `{0}` not found in the rule set")]
    LabelNotFoundInRules(String),
    /// Directory creation failed for `{0}`
    #[error("Directory creation failed for `{0:?}`")]
    DirectoryCreationFailed((String, Box<std::io::Error>)),
    /// Error from the google_gmail1 crate
    #[error(transparent)]
    GoogleGmail1(#[from] Box<google_gmail1::Error>),
    /// Error from std::io
    #[error(transparent)]
    StdIO(#[from] std::io::Error),
    /// Error from toml_de
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
}
