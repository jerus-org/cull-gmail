//! # Gmail Labels CLI Module
//!
//! This module provides command-line interface functionality for inspecting and displaying
//! Gmail labels. It enables users to list all available labels in their Gmail account
//! along with their internal Gmail IDs and display names.
//!
//! ## Purpose
//!
//! The labels command is essential for:
//! - Understanding the structure of Gmail labels in an account
//! - Finding correct label names for use in message queries
//! - Inspecting label IDs for advanced Gmail API usage
//! - Verifying label availability before creating rules
//!
//! ## Usage
//!
//! ```bash
//! cull-gmail labels
//! ```
//!
//! ## Output Format
//!
//! The command displays labels in a human-readable format showing:
//! - **Label Name**: User-visible label name
//! - **Label ID**: Internal Gmail identifier
//!
//! Example output:
//! ```text
//! INBOX: INBOX
//! IMPORTANT: IMPORTANT  
//! promotions: Label_1234567890
//! newsletters: Label_0987654321
//! ```
//!
//! ## Integration
//!
//! This module integrates with:
//! - **GmailClient**: For Gmail API communication and authentication
//! - **Main CLI**: As a subcommand in the primary CLI application
//! - **Error handling**: Using the unified crate error types

use clap::Parser;
use cull_gmail::{Error, GmailClient};

/// Command-line interface for Gmail label inspection and display.
///
/// This structure represents the `labels` subcommand, which provides functionality
/// to list and inspect all Gmail labels available in the user's account. The command
/// requires no additional arguments and displays comprehensive label information.
///
/// # Features
///
/// - **Complete label listing**: Shows all labels including system and user-created labels
/// - **ID mapping**: Displays both human-readable names and internal Gmail IDs
/// - **Simple usage**: No configuration or arguments required
/// - **Authentication handling**: Automatic OAuth2 authentication through GmailClient
///
/// # Usage Context
///
/// This command is typically used:
/// 1. **Before creating queries**: To understand available labels for message filtering
/// 2. **Before configuring rules**: To verify target labels exist
/// 3. **For debugging**: To inspect label structure and IDs
/// 4. **For exploration**: To understand Gmail organization structure
///
/// # Examples
///
/// ```rust,no_run
/// use cull_gmail::cli::labels_cli::LabelsCli;
/// use cull_gmail::GmailClient;
///
/// # async fn example(client: GmailClient) -> Result<(), cull_gmail::Error> {
/// let labels_cli = LabelsCli {};
/// labels_cli.run(client).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Parser)]
pub struct LabelsCli {}

impl LabelsCli {
    /// Executes the labels command to display Gmail label information.
    ///
    /// This method coordinates the label inspection workflow by utilizing the
    /// Gmail client to retrieve and display all available labels in the user's account.
    /// The output includes both system labels (like INBOX, SENT) and user-created labels.
    ///
    /// # Arguments
    ///
    /// * `client` - Authenticated Gmail client for API communication
    ///
    /// # Returns
    ///
    /// Returns `Result<(), Error>` indicating success or failure of the operation.
    ///
    /// # Operation Details
    ///
    /// The function performs the following steps:
    /// 1. **Label Retrieval**: Fetches all labels from the Gmail API
    /// 2. **Format Processing**: Organizes labels for display
    /// 3. **Display Output**: Shows labels with names and IDs
    ///
    /// # Output Format
    ///
    /// Labels are displayed in the format:
    /// ```text
    /// <Label Name>: <Label ID>
    /// ```
    ///
    /// # Error Handling
    ///
    /// Possible errors include:
    /// - **Authentication failures**: OAuth2 token issues or expired credentials
    /// - **API communication errors**: Network issues or Gmail API unavailability
    /// - **Permission errors**: Insufficient OAuth2 scopes for label access
    ///
    /// # Side Effects
    ///
    /// This function produces output to stdout showing the label information.
    /// No Gmail account modifications are performed.
    pub async fn run(&self, client: GmailClient) -> Result<(), Error> {
        client.show_label();
        Ok(())
    }
}
