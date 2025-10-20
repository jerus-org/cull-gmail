//! # Message Summary Module
//!
//! This module provides the `MessageSummary` struct for representing Gmail message metadata
//! in a simplified format suitable for display and processing.

use crate::utils::Elide;

/// A simplified representation of Gmail message metadata.
///
/// `MessageSummary` stores essential message information including ID, subject, and date.
/// It provides methods for accessing this information with fallback text for missing data.
///
/// # Examples
///
/// ```rust
/// # use cull_gmail::gmail_client::message_summary::MessageSummary;
/// let mut summary = MessageSummary::new("message_123");
/// summary.set_subject(Some("Hello World".to_string()));
/// summary.set_date(Some("2023-01-15 10:30:00".to_string()));
///
/// println!("Subject: {}", summary.subject());
/// println!("Date: {}", summary.date());
/// println!("Summary: {}", summary.list_date_and_subject());
/// ```
#[derive(Debug, Clone)]
pub struct MessageSummary {
    id: String,
    date: Option<String>,
    subject: Option<String>,
}

impl MessageSummary {
    /// Creates a new `MessageSummary` with the given message ID.
    ///
    /// The subject and date fields are initialized as `None` and can be set later
    /// using the setter methods.
    ///
    /// # Arguments
    ///
    /// * `id` - The Gmail message ID
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let summary = MessageSummary::new("1234567890abcdef");
    /// assert_eq!(summary.id(), "1234567890abcdef");
    /// ```
    pub(crate) fn new(id: &str) -> Self {
        MessageSummary {
            id: id.to_string(),
            date: None,
            subject: None,
        }
    }

    /// Returns the Gmail message ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let summary = MessageSummary::new("msg_123");
    /// assert_eq!(summary.id(), "msg_123");
    /// ```
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    /// Sets the subject line of the message.
    ///
    /// # Arguments
    ///
    /// * `subject` - Optional subject line text
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let mut summary = MessageSummary::new("msg_123");
    /// summary.set_subject(Some("Important Email".to_string()));
    /// assert_eq!(summary.subject(), "Important Email");
    /// ```
    pub(crate) fn set_subject(&mut self, subject: Option<String>) {
        self.subject = subject
    }

    /// Returns the subject line or a fallback message if none is set.
    ///
    /// # Returns
    ///
    /// The subject line if available, otherwise "*** No Subject for Message ***".
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let mut summary = MessageSummary::new("msg_123");
    /// assert_eq!(summary.subject(), "*** No Subject for Message ***");
    ///
    /// summary.set_subject(Some("Hello".to_string()));
    /// assert_eq!(summary.subject(), "Hello");
    /// ```
    pub(crate) fn subject(&self) -> &str {
        if let Some(s) = &self.subject {
            s
        } else {
            "*** No Subject for Message ***"
        }
    }

    /// Sets the date of the message.
    ///
    /// # Arguments
    ///
    /// * `date` - Optional date string (typically in RFC format)
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let mut summary = MessageSummary::new("msg_123");
    /// summary.set_date(Some("2023-12-25 09:00:00".to_string()));
    /// assert_eq!(summary.date(), "2023-12-25 09:00:00");
    /// ```
    pub(crate) fn set_date(&mut self, date: Option<String>) {
        self.date = date
    }

    /// Returns the message date or a fallback message if none is set.
    ///
    /// # Returns
    ///
    /// The date string if available, otherwise "*** No Date for Message ***".
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let mut summary = MessageSummary::new("msg_123");
    /// assert_eq!(summary.date(), "*** No Date for Message ***");
    ///
    /// summary.set_date(Some("2023-12-25".to_string()));
    /// assert_eq!(summary.date(), "2023-12-25");
    /// ```
    pub(crate) fn date(&self) -> &str {
        if let Some(d) = &self.date {
            d
        } else {
            "*** No Date for Message ***"
        }
    }

    /// Creates a formatted string combining date and subject for list display.
    ///
    /// This method extracts a portion of the date (characters 5-16) and combines it
    /// with an elided version of the subject line for compact display in message lists.
    ///
    /// # Returns
    ///
    /// A formatted string with date and subject, or an error message if either
    /// field is missing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use cull_gmail::gmail_client::message_summary::MessageSummary;
    /// let mut summary = MessageSummary::new("msg_123");
    /// summary.set_date(Some("2023-12-25 09:00:00 GMT".to_string()));
    /// summary.set_subject(Some("This is a very long subject line that will be truncated".to_string()));
    ///
    /// let display = summary.list_date_and_subject();
    /// // Result would be something like: "2-25 09:00: This is a very long s..."
    /// ```
    pub(crate) fn list_date_and_subject(&self) -> String {
        let Some(date) = self.date.as_ref() else {
            return "***invalid date or subject***".to_string();
        };

        let Some(subject) = self.subject.as_ref() else {
            return "***invalid date or subject***".to_string();
        };
        let s = date[5..16].to_string();
        let s = format!("{s}: {}", subject.clone().elide(24));
        s
    }
}
