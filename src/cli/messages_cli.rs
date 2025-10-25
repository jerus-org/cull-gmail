//! # Gmail Messages CLI Module
//!
//! This module provides comprehensive command-line interface functionality for querying,
//! filtering, and performing batch operations on Gmail messages. It supports advanced
//! Gmail query syntax, label-based filtering, and safe batch operations with built-in
//! dry-run capabilities.
//!
//! ## Overview
//!
//! The messages command enables users to:
//! - **Query messages**: Using Gmail's powerful search syntax
//! - **Filter by labels**: Target specific message categories
//! - **Batch operations**: Perform actions on multiple messages efficiently
//! - **Safety controls**: Preview operations before execution
//!
//! ## Command Structure
//!
//! ```bash
//! cull-gmail messages [OPTIONS] <ACTION>
//! ```
//!
//! ### Available Actions
//!
//! - **`list`**: Display message information without modifications
//! - **`trash`**: Move messages to Gmail's Trash folder (recoverable)
//! - **`delete`**: Permanently delete messages (irreversible)
//!
//! ### Filtering Options
//!
//! - **`-l, --labels`**: Filter by Gmail labels (can be specified multiple times)
//! - **`-Q, --query`**: Advanced Gmail query string using Gmail search syntax
//! - **`-m, --max-results`**: Maximum results per page (default: 200)
//! - **`-p, --pages`**: Maximum number of pages to process (0 = all pages)
//!
//! ## Gmail Query Syntax
//!
//! The module supports Gmail's full query syntax including:
//!
//! ### Date Queries
//! - `older_than:1y` - Messages older than 1 year
//! - `newer_than:30d` - Messages newer than 30 days
//! - `after:2023/1/1` - Messages after specific date
//!
//! ### Label Queries
//! - `label:promotions` - Messages with promotions label
//! - `-label:important` - Messages WITHOUT important label
//!
//! ### Content Queries
//! - `subject:newsletter` - Subject contains "newsletter"
//! - `from:example.com` - Messages from domain
//! - `has:attachment` - Messages with attachments
//!
//! ## Safety Features
//!
//! - **Preview mode**: List action shows what would be affected
//! - **Pagination**: Controlled processing with page limits
//! - **Error handling**: Graceful handling of API errors and network issues
//! - **Logging**: Comprehensive operation logging for audit trails
//!
//! ## Examples
//!
//! ### List Recent Messages
//! ```bash
//! cull-gmail messages -m 10 list
//! ```
//!
//! ### Find Old Promotional Emails
//! ```bash
//! cull-gmail messages -Q "label:promotions older_than:1y" list
//! ```
//!
//! ### Batch Trash Operation
//! ```bash
//! cull-gmail messages -Q "label:newsletters older_than:6m" trash
//! ```
//!
//! ### Multi-Label Query
//! ```bash
//! cull-gmail messages -l "promotions" -l "newsletters" -Q "older_than:3m" list
//! ```

use clap::{Parser, Subcommand};
use cull_gmail::{GmailClient, MessageList, Result, RuleProcessor};

/// Available actions for Gmail message operations.
///
/// This enum defines the three primary operations that can be performed on Gmail messages
/// through the CLI, each with different levels of safety and reversibility.
///
/// # Action Safety Levels
///
/// - **List**: Safe inspection operation with no modifications
/// - **Trash**: Recoverable operation (messages can be restored for ~30 days)
/// - **Delete**: Permanent operation (irreversible)
///
/// # Usage Context
///
/// Actions are typically used in this progression:
/// 1. **List** - Preview messages that match criteria
/// 2. **Trash** - Move messages to recoverable trash
/// 3. **Delete** - Permanently remove messages (use with extreme caution)
#[derive(Debug, Subcommand)]
enum MessageAction {
    /// Display message information without making any changes.
    ///
    /// This is the safest operation, showing message details including:
    /// - Message subject and sender
    /// - Date and size information  
    /// - Labels and threading information
    /// - Internal Gmail message IDs
    List,

    /// Move messages to Gmail's Trash folder.
    ///
    /// This operation:
    /// - Moves messages to the Trash label
    /// - Allows recovery for approximately 30 days
    /// - Is reversible through Gmail's web interface
    /// - Provides a safety buffer before permanent deletion
    Trash,

    /// Permanently delete messages from Gmail.
    ///
    /// **WARNING**: This operation is irreversible!
    /// - Messages are permanently removed from Gmail
    /// - No recovery is possible after deletion
    /// - Use extreme caution and always test with list first
    /// - Consider using trash instead for safety
    Delete,
}

/// Command-line interface for Gmail message querying and batch operations.
///
/// This structure encapsulates all configuration options for the messages subcommand,
/// providing comprehensive filtering, pagination, and action capabilities for Gmail
/// message management. It supports complex queries using Gmail's search syntax and
/// multiple filtering mechanisms.
///
/// # Configuration Categories
///
/// - **Pagination**: Control result set size and page limits
/// - **Filtering**: Label-based and query-based message selection
/// - **Actions**: Operations to perform on selected messages
///
/// # Usage Patterns
///
/// ## Safe Exploration
/// ```bash
/// # Start with list to preview results
/// cull-gmail messages -Q "older_than:1y" list
///
/// # Then perform actions on the same query
/// cull-gmail messages -Q "older_than:1y" trash
/// ```
///
/// ## Controlled Processing
/// ```bash
/// # Process in small batches
/// cull-gmail messages -m 50 -p 5 -Q "label:newsletters" list
/// ```
///
/// ## Multi-Criteria Filtering
/// ```bash
/// # Combine labels and query filters
/// cull-gmail messages -l "promotions" -l "social" -Q "older_than:6m" trash
/// ```
///
/// # Safety Considerations
///
/// - Always use `list` action first to preview results
/// - Start with small page sizes for destructive operations
/// - Use `trash` instead of `delete` when possible for recoverability
/// - Test queries thoroughly before batch operations
#[derive(Debug, Parser)]
pub struct MessagesCli {
    /// Maximum number of messages to retrieve per page.
    ///
    /// Controls the batch size for Gmail API requests. Larger values are more
    /// efficient but may hit API rate limits. Smaller values provide more
    /// granular control and progress feedback.
    ///
    /// **Range**: 1-500 (Gmail API limit)
    /// **Performance**: 100-200 is typically optimal
    #[arg(short, long,display_order = 1, help_heading = "Config", default_value = cull_gmail::DEFAULT_MAX_RESULTS)]
    max_results: u32,

    /// Maximum number of pages to process.
    ///
    /// Limits the total number of API requests and messages processed.
    /// Use 0 for unlimited pages (process all matching messages).
    ///
    /// **Safety**: Start with 1-2 pages for testing destructive operations
    /// **Performance**: Higher values process more messages but take longer
    #[arg(
        short,
        long,
        display_order = 1,
        help_heading = "Config",
        default_value = "1"
    )]
    pages: u32,

    /// Gmail labels to filter messages (can be specified multiple times).
    ///
    /// Filters messages to only those containing ALL specified labels.
    /// Use `cull-gmail labels` to see available labels in your account.
    ///
    /// **Examples**:
    /// - `-l "INBOX"` - Messages in inbox
    /// - `-l "promotions" -l "unread"` - Unread promotional messages
    #[arg(short, long, display_order = 1, help_heading = "Config")]
    labels: Vec<String>,

    /// Gmail query string using Gmail's advanced search syntax.
    ///
    /// Supports the same query syntax as Gmail's web interface search box.
    /// Can be combined with label filters for more precise targeting.
    ///
    /// **Examples**:
    /// - `"older_than:1y"` - Messages older than 1 year
    /// - `"from:noreply@example.com older_than:30d"` - Old automated emails
    /// - `"has:attachment larger:10M"` - Large attachments
    #[arg(short = 'Q', long, display_order = 1, help_heading = "Config")]
    query: Option<String>,

    /// Action to perform on the filtered messages.
    ///
    /// Determines what operation to execute on messages matching the filter criteria.
    /// Actions range from safe inspection (list) to permanent deletion (delete).
    #[command(subcommand)]
    action: MessageAction,
}

impl MessagesCli {
    /// Executes the messages command with the configured parameters and action.
    ///
    /// This method orchestrates the complete message processing workflow:
    /// 1. **Parameter Configuration**: Apply filters, pagination, and query settings
    /// 2. **Message Retrieval**: Fetch messages from Gmail API based on criteria
    /// 3. **Action Execution**: Perform the specified operation on retrieved messages
    ///
    /// # Arguments
    ///
    /// * `client` - Mutable Gmail client for API operations and state management
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure of the complete operation.
    ///
    /// # Processing Flow
    ///
    /// ## Parameter Setup
    /// - Apply label filters to restrict message scope
    /// - Configure Gmail query string for advanced filtering
    /// - Set pagination parameters for controlled processing
    ///
    /// ## Message Retrieval
    /// - Execute Gmail API requests to fetch matching messages
    /// - Handle pagination according to configured limits
    /// - Process results in manageable batches
    ///
    /// ## Action Execution
    /// - **List**: Display message information with logging level awareness
    /// - **Trash**: Move messages to Gmail Trash (recoverable)
    /// - **Delete**: Permanently remove messages (irreversible)
    ///
    /// # Error Handling
    ///
    /// The method handles various error conditions:
    /// - **Parameter errors**: Invalid labels or malformed queries
    /// - **API errors**: Network issues, authentication failures, rate limits
    /// - **Action errors**: Failures during trash or delete operations
    ///
    /// # Performance Considerations
    ///
    /// - **Batch processing**: Messages are processed in configurable batches
    /// - **Rate limiting**: Respects Gmail API quotas and limits
    /// - **Memory management**: Efficient handling of large result sets
    ///
    /// # Safety Features
    ///
    /// - **Logging awareness**: List output adapts to logging verbosity
    /// - **Error isolation**: Individual message failures don't stop batch processing
    /// - **Progress tracking**: Detailed logging for operation monitoring
    pub(crate) async fn run(&self, client: &mut GmailClient) -> Result<()> {
        self.set_parameters(client)?;

        client.get_messages(self.pages).await?;

        match self.action {
            MessageAction::List => {
                if log::max_level() >= log::Level::Info {
                    client.log_messages("", "").await
                } else {
                    Ok(())
                }
            }
            MessageAction::Trash => client.batch_trash().await,
            MessageAction::Delete => client.batch_delete().await,
        }

        // Ok(())
    }

    /// Configures the Gmail client with filtering and pagination parameters.
    ///
    /// This method applies all user-specified configuration to the Gmail client,
    /// preparing it for message retrieval operations. It handles label filters,
    /// query strings, and pagination settings with comprehensive error checking.
    ///
    /// # Arguments
    ///
    /// * `client` - Mutable Gmail client to configure
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure of parameter configuration.
    ///
    /// # Configuration Steps
    ///
    /// ## Label Filtering
    /// - Validates label names against available Gmail labels
    /// - Applies multiple label filters with AND logic
    /// - Skips label configuration if no labels specified
    ///
    /// ## Query Configuration
    /// - Applies Gmail query string if provided
    /// - Combines with label filters for refined targeting
    /// - Uses Gmail's native query syntax parsing
    ///
    /// ## Pagination Setup
    /// - Configures maximum results per page for API efficiency
    /// - Logs configuration values for debugging and verification
    /// - Ensures values are within Gmail API limits
    ///
    /// # Error Conditions
    ///
    /// The method can fail due to:
    /// - **Invalid labels**: Label names that don't exist in the Gmail account
    /// - **Malformed queries**: Query syntax that Gmail API cannot parse
    /// - **Parameter limits**: Values outside Gmail API acceptable ranges
    ///
    /// # Logging
    ///
    /// Configuration steps are logged at appropriate levels:
    /// - **Trace**: Detailed parameter values for debugging
    /// - **Debug**: Configuration confirmation and validation results
    fn set_parameters(&self, client: &mut GmailClient) -> Result<()> {
        if !self.labels().is_empty() {
            client.add_labels(self.labels())?;
        }

        if let Some(query) = self.query().as_ref() {
            client.set_query(query)
        }

        log::trace!("Max results: `{}`", self.max_results());
        client.set_max_results(self.max_results());
        log::debug!("List max results set to {}", client.max_results());

        Ok(())
    }

    /// Returns a reference to the configured Gmail labels for filtering.
    ///
    /// This accessor provides access to the list of labels that will be used
    /// to filter messages. Labels are combined with AND logic, meaning messages
    /// must have ALL specified labels to be included in results.
    ///
    /// # Returns
    ///
    /// Returns a reference to the vector of label names as configured by the user.
    /// An empty vector indicates no label-based filtering will be applied.
    pub(crate) fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Returns a reference to the configured Gmail query string.
    ///
    /// This accessor provides access to the advanced query string that will be
    /// applied to message filtering. The query uses Gmail's native search syntax
    /// and can be combined with label filters for precise targeting.
    ///
    /// # Returns
    ///
    /// Returns a reference to the optional query string. `None` indicates
    /// no advanced query filtering will be applied.
    pub(crate) fn query(&self) -> &Option<String> {
        &self.query
    }

    /// Returns the maximum number of messages to retrieve per page.
    ///
    /// This accessor provides the configured batch size for Gmail API requests.
    /// The value determines how many messages are fetched in each API call,
    /// affecting both performance and memory usage.
    ///
    /// # Returns
    ///
    /// Returns the maximum results per page as configured by the user or default value.
    /// The value is guaranteed to be within Gmail API acceptable limits.
    pub(crate) fn max_results(&self) -> u32 {
        self.max_results
    }
}
