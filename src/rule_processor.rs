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
//! ```rust,no_run
//! # use cull_gmail::{GmailClient, RuleProcessor, EolRule, EolAction};
//! # async fn example() -> cull_gmail::Result<()> {
//! let mut client = GmailClient::new().await?;
//!
//! // Create a rule (this would typically come from configuration)
//! let rule = EolRule::new(1, "old-emails".to_string(), EolAction::Trash, None, None, None)?;
//!
//! client.set_rule(rule);
//! client.set_execute(true); // Set to false for dry-run
//!
//! // Process all messages with the "old-emails" label according to the rule
//! client.find_rule_and_messages_for_label("old-emails").await?;
//! # Ok(())
//! # }
//! ```

use google_gmail1::api::{BatchDeleteMessagesRequest, BatchModifyMessagesRequest};

use crate::{EolAction, Error, GmailClient, Result, message_list::MessageList, rules::EolRule};

/// Gmail label name for the trash folder.
///
/// This constant ensures consistent usage of the TRASH label throughout the module.
const TRASH_LABEL: &str = "TRASH";

/// Gmail API scope for modifying messages (recommended scope for most operations).
///
/// This scope allows adding/removing labels, moving messages to trash, and other
/// modification operations. Preferred over broader scopes for security.
const GMAIL_MODIFY_SCOPE: &str = "https://www.googleapis.com/auth/gmail.modify";

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

    /// Configures the end-of-life rule to apply during processing.
    ///
    /// # Arguments
    ///
    /// * `rule` - The `EolRule` containing query criteria and action to perform
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use cull_gmail::{GmailClient, RuleProcessor, EolRule, EolAction};
    /// # fn example() -> cull_gmail::Result<()> {
    /// # let mut client = GmailClient::new_fake(); // This would be real in practice
    /// let rule = EolRule::new(1, "old".to_string(), EolAction::Trash, None, None, None)?;
    /// client.set_rule(rule);
    /// # Ok(())
    /// # }
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
    /// Requires the `https://www.googleapis.com/auth/gmail.modify` scope or broader.
    fn batch_delete(&self) -> impl std::future::Future<Output = Result<()>> + Send;

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
    fn batch_trash(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

impl RuleProcessor for GmailClient {
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
    /// This method implements the main processing logic:
    /// 1. Validates the label exists in the mailbox
    /// 2. Constructs a Gmail query from the rule's criteria
    /// 3. Fetches matching messages from the Gmail API
    /// 4. Executes the rule's action if execute flag is enabled
    ///
    /// The method respects the execute flag - when `false`, it runs in dry-run mode
    /// and only logs what would be done without making any changes.
    async fn find_rule_and_messages_for_label(&mut self, label: &str) -> Result<()> {
        self.add_labels(&[label.to_owned()])?;

        if self.label_ids().is_empty() {
            return Err(Error::LabelNotFoundInMailbox(label.to_string()));
        }

        let Some(rule) = &self.rule else {
            return Err(Error::RuleNotFound(0));
        };

        let Some(query) = rule.eol_query() else {
            return Err(Error::NoQueryStringCalculated(rule.id()));
        };
        self.set_query(&query);

        log::info!("{:?}", self.messages());
        log::info!("Ready to run");
        self.prepare(0).await?;
        if self.execute {
            log::info!("Execute mode: applying rule action to messages");
            self.batch_trash().await
        } else {
            log::info!("Dry-run mode: no changes made to messages");
            Ok(())
        }
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
    async fn batch_delete(&self) -> Result<()> {
        let message_ids = self.message_ids();

        // Early return if no messages to delete, avoiding unnecessary API calls
        if message_ids.is_empty() {
            log::info!("No messages to delete - skipping batch delete operation");
            return Ok(());
        }

        let ids = Some(message_ids);
        let batch_request = BatchDeleteMessagesRequest { ids };

        log::trace!("{batch_request:#?}");

        let _res = self
            .hub()
            .users()
            .messages_batch_delete(batch_request, "me")
            .add_scope(GMAIL_MODIFY_SCOPE)
            .doit()
            .await
            .map_err(Box::new)?;

        for m in self.messages() {
            log::info!("Message with subject `{}` permanently deleted", m.subject());
        }

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
    async fn batch_trash(&self) -> Result<()> {
        let message_ids = self.message_ids();

        // Early return if no messages to trash, avoiding unnecessary API calls
        if message_ids.is_empty() {
            log::info!("No messages to trash - skipping batch trash operation");
            return Ok(());
        }

        let add_label_ids = Some(vec![TRASH_LABEL.to_string()]);
        let ids = Some(message_ids);
        let remove_label_ids = Some(self.label_ids());

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

        for m in self.messages() {
            log::info!("Message with subject `{}` moved to trash", m.subject());
        }

        Ok(())
    }
}
