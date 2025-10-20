//! # End-of-Life Action Module
//!
//! This module defines the actions that can be performed on Gmail messages
//! when they reach their end-of-life criteria based on configured rules.
//!
//! ## Overview
//!
//! The `EolAction` enum specifies how messages should be handled when they
//! meet the criteria for removal from a Gmail account. The module provides
//! two primary actions:
//!
//! - **Trash**: Moves messages to the trash folder (reversible)
//! - **Delete**: Permanently deletes messages (irreversible)
//!
//! ## Safety Considerations
//!
//! - **Trash** action allows message recovery from Gmail's trash folder
//! - **Delete** action permanently removes messages and cannot be undone
//! - Always test rules carefully before applying delete actions
//!
//! ## Usage Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use cull_gmail::EolAction;
//!
//! // Default action is Trash (safer option)
//! let action = EolAction::default();
//! assert_eq!(action, EolAction::Trash);
//!
//! // Parse from string
//! let delete_action = EolAction::parse("delete").unwrap();
//! assert_eq!(delete_action, EolAction::Delete);
//!
//! // Display as string
//! println!("Action: {}", delete_action); // Prints: "Action: delete"
//! ```
//!
//! ### Integration with Rules
//!
//! ```rust,no_run
//! use cull_gmail::EolAction;
//!
//! fn configure_rule_action(action_str: &str) -> Option<EolAction> {
//!     match EolAction::parse(action_str) {
//!         Some(action) => {
//!             println!("Configured action: {}", action);
//!             Some(action)
//!         }
//!         None => {
//!             eprintln!("Invalid action: {}", action_str);
//!             None
//!         }
//!     }
//! }
//! ```
//!
//! ## String Representation
//!
//! The enum implements both parsing from strings and display formatting:
//!
//! | Variant | String | Description |
//! |---------|--------|--------------|
//! | `Trash` | "trash" | Move to trash (recoverable) |
//! | `Delete` | "delete" | Permanent deletion |
//!
//! Parsing is case-insensitive, so "TRASH", "Trash", and "trash" are all valid.

use std::fmt;

/// Represents the action to take on Gmail messages that meet end-of-life criteria.
///
/// This enum defines the two possible actions for handling messages when they
/// reach the end of their lifecycle based on configured retention rules.
///
/// # Variants
///
/// - [`Trash`](EolAction::Trash) - Move messages to Gmail's trash folder (default, reversible)
/// - [`Delete`](EolAction::Delete) - Permanently delete messages (irreversible)
///
/// # Default Behavior
///
/// The default action is [`Trash`](EolAction::Trash), which provides a safety net
/// by allowing message recovery from the trash folder.
///
/// # Examples
///
/// ```rust
/// use cull_gmail::EolAction;
///
/// // Using the default (Trash)
/// let safe_action = EolAction::default();
/// assert_eq!(safe_action, EolAction::Trash);
///
/// // Comparing actions
/// let delete = EolAction::Delete;
/// let trash = EolAction::Trash;
/// assert_ne!(delete, trash);
///
/// // Converting to string for logging/display
/// println!("Action: {}", delete); // Prints: "delete"
/// ```
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum EolAction {
    /// Move the message to Gmail's trash folder.
    ///
    /// This is the default and safer option as it allows message recovery.
    /// Messages in the trash are automatically deleted by Gmail after 30 days.
    ///
    /// # Safety
    ///
    /// This action is reversible - messages can be recovered from the trash folder
    /// until they are automatically purged or manually deleted from trash.
    #[default]
    Trash,
    
    /// Permanently delete the message immediately.
    ///
    /// This action bypasses the trash folder and permanently removes the message.
    /// 
    /// # Warning
    ///
    /// This action is **irreversible**. Once deleted, messages cannot be recovered.
    /// Use with extreme caution and thorough testing of rule criteria.
    ///
    /// # Use Cases
    ///
    /// - Sensitive data that should not remain in trash
    /// - Storage optimization where trash recovery is not needed
    /// - Automated cleanup of known disposable messages
    Delete,
}

impl fmt::Display for EolAction {
    /// Formats the `EolAction` as a lowercase string.
    ///
    /// This implementation provides a consistent string representation
    /// for logging, configuration, and user interfaces.
    ///
    /// # Returns
    ///
    /// - `"trash"` for [`EolAction::Trash`]
    /// - `"delete"` for [`EolAction::Delete`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::EolAction;
    ///
    /// assert_eq!(EolAction::Trash.to_string(), "trash");
    /// assert_eq!(EolAction::Delete.to_string(), "delete");
    ///
    /// // Useful for logging
    /// let action = EolAction::default();
    /// println!("Performing action: {}", action);
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EolAction::Trash => write!(f, "trash"),
            EolAction::Delete => write!(f, "delete"),
        }
    }
}

impl EolAction {
    /// Parses a string into an `EolAction` variant.
    ///
    /// This method provides case-insensitive parsing from string representations
    /// to `EolAction` variants. It's useful for configuration file parsing,
    /// command-line arguments, and user input validation.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice to parse. Case is ignored.
    ///
    /// # Returns
    ///
    /// - `Some(EolAction)` if the string matches a valid variant
    /// - `None` if the string is not recognized
    ///
    /// # Valid Input Strings
    ///
    /// - `"trash"`, `"Trash"`, `"TRASH"` → [`EolAction::Trash`]
    /// - `"delete"`, `"Delete"`, `"DELETE"` → [`EolAction::Delete`]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::EolAction;
    ///
    /// // Valid parsing (case-insensitive)
    /// assert_eq!(EolAction::parse("trash"), Some(EolAction::Trash));
    /// assert_eq!(EolAction::parse("TRASH"), Some(EolAction::Trash));
    /// assert_eq!(EolAction::parse("Delete"), Some(EolAction::Delete));
    ///
    /// // Invalid input
    /// assert_eq!(EolAction::parse("invalid"), None);
    /// assert_eq!(EolAction::parse(""), None);
    /// ```
    ///
    /// # Use Cases
    ///
    /// ```rust
    /// use cull_gmail::EolAction;
    ///
    /// fn parse_user_action(input: &str) -> Result<EolAction, String> {
    ///     EolAction::parse(input)
    ///         .ok_or_else(|| format!("Invalid action: '{}'. Use 'trash' or 'delete'.", input))
    /// }
    ///
    /// assert!(parse_user_action("trash").is_ok());
    /// assert!(parse_user_action("invalid").is_err());
    /// ```
    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "trash" => Some(EolAction::Trash),
            "delete" => Some(EolAction::Delete),
            _ => None,
        }
    }

    /// Returns `true` if the action is reversible (can be undone).
    ///
    /// This method helps determine if an action allows for message recovery,
    /// which is useful for safety checks and user confirmations.
    ///
    /// # Returns
    ///
    /// - `true` for [`EolAction::Trash`] (messages can be recovered from trash)
    /// - `false` for [`EolAction::Delete`] (messages are permanently deleted)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::EolAction;
    ///
    /// assert!(EolAction::Trash.is_reversible());
    /// assert!(!EolAction::Delete.is_reversible());
    ///
    /// // Use in safety checks
    /// let action = EolAction::Delete;
    /// if !action.is_reversible() {
    ///     println!("Warning: This action cannot be undone!");
    /// }
    /// ```
    pub fn is_reversible(&self) -> bool {
        match self {
            EolAction::Trash => true,
            EolAction::Delete => false,
        }
    }

    /// Returns all possible `EolAction` variants.
    ///
    /// This method is useful for generating help text, validation lists,
    /// or iterating over all possible actions.
    ///
    /// # Returns
    ///
    /// An array containing all `EolAction` variants in declaration order.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::EolAction;
    ///
    /// let all_actions = EolAction::variants();
    /// assert_eq!(all_actions.len(), 2);
    /// assert_eq!(all_actions[0], EolAction::Trash);
    /// assert_eq!(all_actions[1], EolAction::Delete);
    ///
    /// // Generate help text
    /// println!("Available actions:");
    /// for action in EolAction::variants() {
    ///     println!("  {} - {}", action, 
    ///              if action.is_reversible() { "reversible" } else { "irreversible" });
    /// }
    /// ```
    pub fn variants() -> &'static [EolAction] {
        &[EolAction::Trash, EolAction::Delete]
    }
}
