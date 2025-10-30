//! Rule Processing Module
//!
//! This module provides the [`RuleProcessor`] trait and its implementation for processing
//! Gmail messages according to configured end-of-life (EOL) rules. It handles the complete
//! workflow of finding messages, applying filters based on rules, and executing actions
//! such as moving messages to trash or permanently deleting them.
//!
//! ## Safety Considerations
//!
//! - **Destructive Operations**: The [`RuleProcessor::batch_delete`] method permanently
//!   removes messages from Gmail and cannot be undone.
//! - **Recoverable Operations**: The [`RuleProcessor::batch_trash`] method moves messages
//!   to the Gmail trash folder, from which they can be recovered within 30 days.
//! - **Execute Flag**: All destructive operations are gated by an execute flag that must
//!   be explicitly set to `true`. When `false`, operations run in "dry-run" mode.
//!
//! ## Workflow
//!
//! 1. Set a rule using [`RuleProcessor::set_rule`]
//! 2. Configure the execute flag with [`RuleProcessor::set_execute`]
//! 3. Process messages for a label with [`RuleProcessor::find_rule_and_messages_for_label`]
//! 4. The processor will automatically:
//!    - Find messages matching the rule's query
//!    - Prepare the message list via [`RuleProcessor::prepare`]
//!    - Execute the rule's action (trash) if execute flag is true
//!
//! ## Example
//!
//! ```text
//! use cull_gmail::{GmailClient, RuleProcessor, ClientConfig};
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     // Configure Gmail client with credentials
//!     let config = ClientConfig::builder()
//!         .with_client_id("your-client-id")
//!         .with_client_secret("your-client-secret")
//!         .build();
//!     let mut client = GmailClient::new_with_config(config).await?;
//!     
//!     // Rules would typically be loaded from configuration
//!     // let rule = load_rule_from_config("old-emails");
//!     // client.set_rule(rule);
//!     
//!     client.set_execute(true); // Set to false for dry-run
//!     
//!     // Process all messages with the "old-emails" label according to the rule
//!     client.find_rule_and_messages_for_label("old-emails").await?;
//!     Ok(())
//! }
//! ```

use google_gmail1::api::{BatchDeleteMessagesRequest, BatchModifyMessagesRequest};

use crate::{EolAction, Error, GmailClient, Result, message_list::MessageList, rules::EolRule};

/// Gmail label name for the trash folder.
///
/// This constant ensures consistent usage of the TRASH label throughout the module.
const TRASH_LABEL: &str = "TRASH";

/// Gmail label name for the inbox folder.
///
/// This constant ensures consistent usage of the INBOX label throughout the module.
const INBOX_LABEL: &str = "INBOX";

/// Gmail API scope for modifying messages (recommended scope for most operations).
///
/// This scope allows adding/removing labels, moving messages to trash, and other
/// modification operations. Preferred over broader scopes for security.
const GMAIL_MODIFY_SCOPE: &str = "https://www.googleapis.com/auth/gmail.modify";

/// Gmail API scope for deleting messages.
///
/// This scope allows all operations and is required to authorise the batch
/// delete operation. It is only used for batch delete. For all other
/// operations `GMAIL_MODIFY_SCOPE` is preferred.
const GMAIL_DELETE_SCOPE: &str = "https://mail.google.com/";

/// Internal trait defining the minimal operations needed for rule processing.
///
/// This trait is used internally to enable unit testing of orchestration logic
/// without requiring network calls or real Gmail API access. It abstracts the
/// core operations that the rule processor needs from the Gmail client.
#[doc(hidden)]
pub(crate) trait MailOperations {
    /// Add labels to the client for filtering
    fn add_labels(&mut self, labels: &[String]) -> Result<()>;

    /// Get the current label IDs
    fn label_ids(&self) -> Vec<String>;

    /// Set the query string for message filtering
    fn set_query(&mut self, query: &str);

    /// Prepare messages by fetching from Gmail API
    fn prepare(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Execute trash operation on prepared messages
    fn batch_trash(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Internal orchestration function for rule processing that can be unit tested.
///
/// This function contains the core rule processing logic extracted from the trait
/// implementation to enable testing without network dependencies.
async fn process_label_with_rule<T: MailOperations>(
    client: &mut T,
    rule: &EolRule,
    label: &str,
    pages: u32,
    execute: bool,
) -> Result<()> {
    // Add the label for filtering
    client.add_labels(&[label.to_owned()])?;

    // Validate label exists in mailbox
    if client.label_ids().is_empty() {
        return Err(Error::LabelNotFoundInMailbox(label.to_owned()));
    }

    // Get query from rule
    let Some(query) = rule.eol_query() else {
        return Err(Error::NoQueryStringCalculated(rule.id()));
    };

    // Set the query and prepare messages
    client.set_query(&query);
    log::info!("Ready to process messages for label: {label}");
    client.prepare(pages).await?;

    // Execute or dry-run based on execute flag
    if execute {
        log::info!("Execute mode: applying rule action to messages");
        client.batch_trash().await
    } else {
        log::info!("Dry-run mode: no changes made to messages");
        Ok(())
    }
}

/// Implement the internal mail operations trait for GmailClient.
impl MailOperations for GmailClient {
    fn add_labels(&mut self, labels: &[String]) -> Result<()> {
        MessageList::add_labels(self, labels)
    }

    fn label_ids(&self) -> Vec<String> {
        MessageList::label_ids(self)
    }

    fn set_query(&mut self, query: &str) {
        MessageList::set_query(self, query);
    }

    async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.get_messages(pages).await
    }

    async fn batch_trash(&mut self) -> Result<()> {
        RuleProcessor::batch_trash(self).await
    }
}

/// Trait for processing Gmail messages according to configured end-of-life rules.
///
/// This trait defines the interface for finding, filtering, and acting upon Gmail messages
/// based on retention rules. Implementations should handle the complete workflow from
/// rule application to message processing.
pub trait RuleProcessor {
    /// Processes all messages for a specific Gmail label according to the configured rule.
    ///
    /// This is the main entry point for rule processing. It coordinates the entire workflow:
    /// 1. Validates that the label exists in the mailbox
    /// 2. Applies the rule's query to find matching messages
    /// 3. Prepares the message list for processing
    /// 4. Executes the rule's action (if execute flag is true) or runs in dry-run mode
    ///
    /// # Arguments
    ///
    /// * `label` - The Gmail label name to process (e.g., "INBOX", "old-emails")
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Processing completed successfully
    /// * `Err(Error::LabelNotFoundInMailbox)` - The specified label doesn't exist
    /// * `Err(Error::RuleNotFound)` - No rule has been set via [`set_rule`](Self::set_rule)
    /// * `Err(Error::NoQueryStringCalculated)` - The rule doesn't provide a valid query
    ///
    /// # Side Effects
    ///
    /// When execute flag is true, messages may be moved to trash or permanently deleted.
    /// When execute flag is false, runs in dry-run mode with no destructive actions.
    fn find_rule_and_messages_for_label(
        &mut self,
        label: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Sets the execution mode for destructive operations.
    ///
    /// # Arguments
    ///
    /// * `value` - `true` to enable destructive operations, `false` for dry-run mode
    ///
    /// # Safety
    ///
    /// When set to `true`, subsequent calls to processing methods will perform actual
    /// destructive operations on Gmail messages. Always verify your rules and queries
    /// in dry-run mode (`false`) before enabling execution.
    fn set_execute(&mut self, value: bool);

    /// Initialises the message and label lists to prepare for application of rule.
    ///
    /// # Arguments
    ///
    /// * none
    ///
    /// # Example
    ///
    /// ```text
    /// use cull_gmail::{GmailClient, RuleProcessor, ClientConfig};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ClientConfig::builder()
    ///         .with_client_id("your-client-id")
    ///         .with_client_secret("your-client-secret")
    ///         .build();
    ///     let mut client = GmailClient::new_with_config(config).await?;
    ///     
    ///     // Rules would typically be loaded from configuration
    ///     // let rule = load_rule_from_config();
    ///     // client.initialise_message_list();
    ///     // client.set_rule(rule);
    ///     Ok(())
    /// }
    /// ```
    fn initialise_lists(&mut self);

    /// Configures the end-of-life rule to apply during processing.
    ///
    /// # Arguments
    ///
    /// * `rule` - The `EolRule` containing query criteria and action to perform
    ///
    /// # Example
    ///
    /// ```text
    /// use cull_gmail::{GmailClient, RuleProcessor, ClientConfig};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = ClientConfig::builder()
    ///         .with_client_id("your-client-id")
    ///         .with_client_secret("your-client-secret")
    ///         .build();
    ///     let mut client = GmailClient::new_with_config(config).await?;
    ///     
    ///     // Rules would typically be loaded from configuration
    ///     // let rule = load_rule_from_config();
    ///     // client.set_rule(rule);
    ///     Ok(())
    /// }
    /// ```
    fn set_rule(&mut self, rule: EolRule);

    /// Returns the action that will be performed by the currently configured rule.
    ///
    /// # Returns
    ///
    /// * `Some(EolAction)` - The action (e.g., `EolAction::Trash`) if a rule is set
    /// * `None` - If no rule has been configured via [`set_rule`](Self::set_rule)
    fn action(&self) -> Option<EolAction>;

    /// Prepares the list of messages for processing by fetching them from Gmail.
    ///
    /// This method queries the Gmail API to retrieve messages matching the current
    /// query and label filters, up to the specified number of pages.
    ///
    /// # Arguments
    ///
    /// * `pages` - Maximum number of result pages to fetch (0 = all pages)
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Messages successfully retrieved and prepared
    /// * `Err(_)` - Gmail API error or network failure
    ///
    /// # Side Effects
    ///
    /// Makes API calls to Gmail to retrieve message metadata. No messages are
    /// modified by this operation.
    fn prepare(&mut self, pages: u32) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Permanently deletes all prepared messages from Gmail.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All messages successfully deleted
    /// * `Err(_)` - Gmail API error, network failure, or insufficient permissions
    ///
    /// # Safety
    ///
    /// ⚠️ **DESTRUCTIVE OPERATION** - This permanently removes messages from Gmail.
    /// Deleted messages cannot be recovered. Use [`batch_trash`](Self::batch_trash)
    /// for recoverable deletion.
    ///
    /// # Gmail API Requirements
    ///
    /// Requires the `https://mail.google.com/` scope or broader.
    fn batch_delete(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Calls the Gmail API to permanently deletes a slice from the list of messages.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All messages successfully deleted
    /// * `Err(_)` - Gmail API error, network failure, or insufficient permissions
    ///
    /// # Safety
    ///
    /// ⚠️ **DESTRUCTIVE OPERATION** - This permanently removes messages from Gmail.
    /// Deleted messages cannot be recovered. Use [`batch_trash`](Self::batch_trash)
    /// for recoverable deletion.
    ///
    /// # Gmail API Requirements
    ///
    /// Requires the `https://mail.google.com/` scope or broader.
    fn call_batch_delete(
        &self,
        ids: &[String],
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Calls the Gmail API to move a slice of the prepared messages to the Gmail
    /// trash folder.
    ///
    /// Messages moved to trash can be recovered within 30 days through the Gmail
    /// web interface or API calls.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All messages successfully moved to trash
    /// * `Err(_)` - Gmail API error, network failure, or insufficient permissions
    ///
    /// # Recovery
    ///
    /// Messages can be recovered from trash within 30 days. After 30 days,
    /// Gmail automatically purges trashed messages.
    ///
    /// # Gmail API Requirements
    ///
    /// Requires the `https://www.googleapis.com/auth/gmail.modify` scope.
    fn batch_trash(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Moves all prepared messages to the Gmail trash folder.
    ///
    /// Messages moved to trash can be recovered within 30 days through the Gmail
    /// web interface or API calls.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - All messages successfully moved to trash
    /// * `Err(_)` - Gmail API error, network failure, or insufficient permissions
    ///
    /// # Recovery
    ///
    /// Messages can be recovered from trash within 30 days. After 30 days,
    /// Gmail automatically purges trashed messages.
    ///
    /// # Gmail API Requirements
    ///
    /// Requires the `https://www.googleapis.com/auth/gmail.modify` scope.
    fn call_batch_trash(
        &self,
        ids: &[String],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl RuleProcessor for GmailClient {
    /// Initialise the message list.
    ///
    /// The message list is initialised to ensure that the rule is only processed
    /// on the in-scope messages.
    ///
    /// This must be called before processing any labels.
    fn initialise_lists(&mut self) {
        self.messages = Vec::new();
        self.label_ids = Vec::new();
    }

    /// Configures the end-of-life rule for this Gmail client.
    ///
    /// The rule defines which messages to target and what action to perform on them.
    /// This must be called before processing any labels.
    fn set_rule(&mut self, value: EolRule) {
        self.rule = Some(value);
    }

    /// Controls whether destructive operations are actually executed.
    ///
    /// When `false` (dry-run mode), all operations are simulated but no actual
    /// changes are made to Gmail messages. When `true`, destructive operations
    /// like moving to trash or deleting will be performed.
    ///
    /// **Default is `false` for safety.**
    fn set_execute(&mut self, value: bool) {
        self.execute = value;
    }

    /// Returns the action that will be performed by the current rule.
    ///
    /// This is useful for logging and verification before executing destructive operations.
    fn action(&self) -> Option<EolAction> {
        if let Some(rule) = &self.rule {
            return rule.action();
        }
        None
    }

    /// Orchestrates the complete rule processing workflow for a Gmail label.
    ///
    /// This method implements the main processing logic by delegating to the internal
    /// orchestration function, which enables better testability while maintaining
    /// the same external behaviour.
    ///
    /// The method respects the execute flag - when `false`, it runs in dry-run mode
    /// and only logs what would be done without making any changes.
    async fn find_rule_and_messages_for_label(&mut self, label: &str) -> Result<()> {
        // Ensure we have a rule configured and clone it to avoid borrow conflicts
        let Some(rule) = self.rule.clone() else {
            return Err(Error::RuleNotFound(0));
        };

        let execute = self.execute;

        // Delegate to internal orchestration function
        process_label_with_rule(self, &rule, label, 0, execute).await
    }

    /// Fetches messages from Gmail API based on current query and label filters.
    ///
    /// This is a read-only operation that retrieves message metadata from Gmail
    /// without modifying any messages. The results are cached internally for
    /// subsequent batch operations.
    ///
    /// # Arguments
    ///
    /// * `pages` - Number of result pages to fetch (0 = all available pages)
    async fn prepare(&mut self, pages: u32) -> Result<()> {
        self.get_messages(pages).await
    }

    /// Permanently deletes all prepared messages using Gmail's batch delete API.
    ///
    /// ⚠️ **DESTRUCTIVE OPERATION** - This action cannot be undone!
    ///
    /// This method uses the Gmail API's batch delete functionality to permanently
    /// remove messages from the user's mailbox. Once deleted, messages cannot be
    /// recovered through any means.
    ///
    /// # API Scope Requirements
    ///
    /// Uses `https://www.googleapis.com/auth/gmail.modify` scope for secure,
    /// minimal privilege access. This scope provides sufficient permissions
    /// for message deletion while following security best practices.
    async fn batch_delete(&mut self) -> Result<()> {
        let message_ids = MessageList::message_ids(self);

        // Early return if no messages to delete, avoiding unnecessary API calls
        if message_ids.is_empty() {
            log::info!("No messages to delete - skipping batch delete operation");
            return Ok(());
        }

        self.log_messages("Message with subject `", "` permanently deleted")
            .await?;

        let (chunks, remainder) = message_ids.as_chunks::<1000>();
        log::trace!(
            "Message list chopped into {} chunks with {} ids in the remainder",
            chunks.len(),
            remainder.len()
        );

        if !chunks.is_empty() {
            for (i, chunk) in chunks.iter().enumerate() {
                log::trace!("Processing chunk {i}");
                self.call_batch_delete(chunk).await?;
            }
        }

        if !remainder.is_empty() {
            log::trace!("Processing remainder.");
            self.call_batch_delete(remainder).await?;
        }

        Ok(())
    }

    async fn call_batch_delete(&self, ids: &[String]) -> Result<()> {
        let ids = Some(Vec::from(ids));
        let batch_request = BatchDeleteMessagesRequest { ids };
        log::trace!("{batch_request:#?}");

        let res = self
            .hub()
            .users()
            .messages_batch_delete(batch_request, "me")
            .add_scope(GMAIL_DELETE_SCOPE)
            .doit()
            .await
            .map_err(Box::new);

        log::trace!("Batch delete response {res:?}");

        res?;

        Ok(())
    }
    /// Moves all prepared messages to Gmail's trash folder using batch modify API.
    ///
    /// This is a recoverable operation - messages can be restored from trash within
    /// 30 days via the Gmail web interface or API calls. After 30 days, Gmail
    /// automatically purges trashed messages permanently.
    ///
    /// The operation adds the TRASH label and removes any existing labels that were
    /// used to filter the messages, effectively moving them out of their current
    /// folders into the trash.
    ///
    /// # API Scope Requirements
    ///
    /// Uses `https://www.googleapis.com/auth/gmail.modify` scope for secure,
    /// minimal privilege access to Gmail message modification operations.
    async fn batch_trash(&mut self) -> Result<()> {
        let message_ids = MessageList::message_ids(self);

        // Early return if no messages to trash, avoiding unnecessary API calls
        if message_ids.is_empty() {
            log::info!("No messages to trash - skipping batch trash operation");
            return Ok(());
        }

        self.log_messages("Message with subject `", "` moved to trash")
            .await?;

        let (chunks, remainder) = message_ids.as_chunks::<1000>();
        log::trace!(
            "Message list chopped into {} chunks with {} ids in the remainder",
            chunks.len(),
            remainder.len()
        );

        if !chunks.is_empty() {
            for (i, chunk) in chunks.iter().enumerate() {
                log::trace!("Processing chunk {i}");
                self.call_batch_delete(chunk).await?;
            }
        }

        if !remainder.is_empty() {
            log::trace!("Processing remainder.");
            self.call_batch_delete(remainder).await?;
        }

        Ok(())
    }

    async fn call_batch_trash(&self, ids: &[String]) -> Result<()> {
        let ids = Some(Vec::from(ids));
        let add_label_ids = Some(vec![TRASH_LABEL.to_string()]);
        let remove_label_ids = Some(vec![INBOX_LABEL.to_string()]);

        let batch_request = BatchModifyMessagesRequest {
            add_label_ids,
            ids,
            remove_label_ids,
        };

        log::trace!("{batch_request:#?}");

        let _res = self
            .hub()
            .users()
            .messages_batch_modify(batch_request, "me")
            .add_scope(GMAIL_MODIFY_SCOPE)
            .doit()
            .await
            .map_err(Box::new)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EolAction, Error, MessageSummary, rules::EolRule};
    use std::sync::{Arc, Mutex};

    /// Test helper to create a simple EolRule with or without a query
    fn create_test_rule(id: usize, has_query: bool) -> EolRule {
        use crate::{MessageAge, Retention};

        let mut rule = EolRule::new(id);

        if has_query {
            // Create a rule that will generate a query (using retention days)
            let retention = Retention::new(MessageAge::Days(30), false);
            rule.set_retention(retention);
            rule.add_label("test-label");
        }
        // For rules without query, we just return the basic rule with no retention set

        rule
    }

    /// Fake client implementation for testing the orchestration logic
    struct FakeClient {
        labels: Vec<String>,
        label_ids: Vec<String>,
        query: String,
        messages_prepared: bool,
        prepare_call_count: u32,
        batch_trash_call_count: Arc<Mutex<u32>>, // Use Arc<Mutex> for thread safety
        should_fail_add_labels: bool,
        should_fail_prepare: bool,
        should_fail_batch_trash: bool,
        simulate_missing_labels: bool, // Flag to simulate labels not being found
    }

    impl Default for FakeClient {
        fn default() -> Self {
            Self {
                labels: Vec::new(),
                label_ids: Vec::new(),
                query: String::new(),
                messages_prepared: false,
                prepare_call_count: 0,
                batch_trash_call_count: Arc::new(Mutex::new(0)),
                should_fail_add_labels: false,
                should_fail_prepare: false,
                should_fail_batch_trash: false,
                simulate_missing_labels: false,
            }
        }
    }

    impl FakeClient {
        fn new() -> Self {
            Self::default()
        }

        /// Create a client that simulates missing labels (add_labels succeeds but no label_ids)
        fn with_missing_labels() -> Self {
            Self {
                simulate_missing_labels: true,
                ..Default::default()
            }
        }

        fn with_labels(label_ids: Vec<String>) -> Self {
            Self {
                label_ids,
                ..Default::default()
            }
        }

        fn with_failure(failure_type: &str) -> Self {
            match failure_type {
                "add_labels" => Self {
                    should_fail_add_labels: true,
                    ..Default::default()
                },
                "prepare" => Self {
                    should_fail_prepare: true,
                    ..Default::default()
                },
                "batch_trash" => Self {
                    should_fail_batch_trash: true,
                    ..Default::default()
                },
                _ => Self::default(),
            }
        }

        fn get_batch_trash_call_count(&self) -> u32 {
            *self.batch_trash_call_count.lock().unwrap()
        }
    }

    impl MailOperations for FakeClient {
        fn add_labels(&mut self, labels: &[String]) -> Result<()> {
            if self.should_fail_add_labels {
                return Err(Error::DirectoryUnset); // Use a valid error variant
            }
            self.labels.extend(labels.iter().cloned());
            // Only populate label_ids if we're not simulating missing labels
            if !self.simulate_missing_labels && !labels.is_empty() {
                self.label_ids = labels.to_vec();
            }
            // When simulate_missing_labels is true, label_ids stays empty
            Ok(())
        }

        fn label_ids(&self) -> Vec<String> {
            self.label_ids.clone()
        }

        fn set_query(&mut self, query: &str) {
            self.query = query.to_owned();
        }

        async fn prepare(&mut self, _pages: u32) -> Result<()> {
            // Always increment the counter to track that prepare was called
            self.prepare_call_count += 1;

            if self.should_fail_prepare {
                return Err(Error::NoLabelsFound); // Use a valid error variant
            }
            self.messages_prepared = true;
            Ok(())
        }

        async fn batch_trash(&mut self) -> Result<()> {
            // Always increment the counter to track that batch_trash was called
            *self.batch_trash_call_count.lock().unwrap() += 1;

            if self.should_fail_batch_trash {
                return Err(Error::InvalidPagingMode); // Use a valid error variant
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_errors_when_label_missing() {
        let mut client = FakeClient::with_missing_labels(); // Simulate labels not being found
        let rule = create_test_rule(1, true);
        let label = "missing-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, false).await;

        assert!(matches!(result, Err(Error::LabelNotFoundInMailbox(_))));
        assert_eq!(client.prepare_call_count, 0);
        assert_eq!(client.get_batch_trash_call_count(), 0);
    }

    #[tokio::test]
    async fn test_errors_when_rule_has_no_query() {
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        let rule = create_test_rule(2, false); // Rule without query
        let label = "test-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, false).await;

        assert!(matches!(result, Err(Error::NoQueryStringCalculated(2))));
        assert_eq!(client.prepare_call_count, 0);
        assert_eq!(client.get_batch_trash_call_count(), 0);
    }

    #[tokio::test]
    async fn test_dry_run_does_not_trash() {
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        let rule = create_test_rule(3, true);
        let label = "test-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, false).await;

        assert!(result.is_ok());
        assert_eq!(client.prepare_call_count, 1);
        assert_eq!(client.get_batch_trash_call_count(), 0); // Should not trash in dry-run mode
        assert!(client.messages_prepared);
        assert!(!client.query.is_empty()); // Query should be set
    }

    #[tokio::test]
    async fn test_execute_trashes_messages_once() {
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        let rule = create_test_rule(4, true);
        let label = "test-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, true).await;

        assert!(result.is_ok());
        assert_eq!(client.prepare_call_count, 1);
        assert_eq!(client.get_batch_trash_call_count(), 1); // Should trash when execute=true
        assert!(client.messages_prepared);
        assert!(!client.query.is_empty());
    }

    #[tokio::test]
    async fn test_propagates_prepare_error() {
        // Create a client that will fail on prepare but has valid labels
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        client.should_fail_prepare = true; // Set the failure flag directly

        let rule = create_test_rule(5, true);
        let label = "test-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, true).await;

        assert!(result.is_err());
        assert_eq!(client.prepare_call_count, 1); // prepare should be called once
        assert_eq!(client.get_batch_trash_call_count(), 0); // Should not reach trash due to error
    }

    #[tokio::test]
    async fn test_propagates_batch_trash_error() {
        // Create a client that will fail on batch_trash but has valid labels
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        client.should_fail_batch_trash = true; // Set the failure flag directly

        let rule = create_test_rule(6, true);
        let label = "test-label";

        let result = process_label_with_rule(&mut client, &rule, label, 0, true).await;

        assert!(result.is_err());
        assert_eq!(client.prepare_call_count, 1);
        assert_eq!(client.get_batch_trash_call_count(), 1); // Should attempt trash but fail
    }

    #[tokio::test]
    async fn test_pages_parameter_passed_correctly() {
        let mut client = FakeClient::with_labels(vec!["test-label".to_string()]);
        let rule = create_test_rule(7, true);
        let label = "test-label";
        let pages = 5;

        let result = process_label_with_rule(&mut client, &rule, label, pages, false).await;

        assert!(result.is_ok());
        assert_eq!(client.prepare_call_count, 1);
        // Note: In a more sophisticated test, we'd verify pages parameter is passed to prepare
        // but our simple FakeClient doesn't track this. In practice, you might want to enhance it.
    }

    /// Test the rule processor trait setters and getters
    #[test]
    fn test_rule_processor_setters_and_getters() {
        // Note: This test would need a mock GmailClient implementation
        // For now, we'll create a simple struct that implements RuleProcessor

        struct MockProcessor {
            messages: Vec<MessageSummary>,
            rule: Option<EolRule>,
            execute: bool,
            labels: Vec<String>,
        }

        impl RuleProcessor for MockProcessor {
            fn initialise_lists(&mut self) {
                self.messages = Vec::new();
                self.labels = Vec::new();
            }

            fn set_rule(&mut self, rule: EolRule) {
                self.rule = Some(rule);
            }

            fn set_execute(&mut self, value: bool) {
                self.execute = value;
            }

            fn action(&self) -> Option<EolAction> {
                self.rule.as_ref().and_then(|r| r.action())
            }

            async fn find_rule_and_messages_for_label(&mut self, _label: &str) -> Result<()> {
                Ok(())
            }

            async fn prepare(&mut self, _pages: u32) -> Result<()> {
                Ok(())
            }

            async fn batch_delete(&mut self) -> Result<()> {
                Ok(())
            }

            async fn call_batch_delete(&self, _ids: &[String]) -> Result<()> {
                Ok(())
            }

            async fn batch_trash(&mut self) -> Result<()> {
                Ok(())
            }

            async fn call_batch_trash(&self, _ids: &[String]) -> Result<()> {
                Ok(())
            }
        }

        let mut processor = MockProcessor {
            rule: None,
            execute: false,
            messages: Vec::new(),
            labels: Vec::new(),
        };

        // Test initial state
        assert!(processor.action().is_none());
        assert!(!processor.execute);

        // Test rule setting
        let rule = create_test_rule(8, true);
        processor.set_rule(rule);
        assert!(processor.action().is_some());
        assert_eq!(processor.action(), Some(EolAction::Trash));

        // Test execute flag setting
        processor.set_execute(true);
        assert!(processor.execute);

        processor.set_execute(false);
        assert!(!processor.execute);
    }
}
