use std::fmt;

/// End of life action
/// - Trash - move the message to the trash to be automatically deleted by Google
/// - Delete - delete the message immediately without allowing rescue from trash
#[derive(Debug, Default)]
pub enum EolAction {
    #[default]
    /// Move the message to the trash
    Trash,
    /// Delete the message immediately
    Delete,
}

impl fmt::Display for EolAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EolAction::Trash => write!(f, "trash"),
            EolAction::Delete => write!(f, "delete"),
        }
    }
}
