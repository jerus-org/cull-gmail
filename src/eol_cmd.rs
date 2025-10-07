use std::fmt;

/// End of life command
/// - Trash - move the message to the trash to be automatically deleted by Google
/// - Delete - delete the message immediately without allowing rescue from trash
#[derive(Debug, Default)]
pub enum EolCmd {
    #[default]
    /// Move the message to the trash
    Trash,
    /// Delete the message immediately
    Delete,
}

impl fmt::Display for EolCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EolCmd::Trash => write!(f, "trash"),
            EolCmd::Delete => write!(f, "delete"),
        }
    }
}
