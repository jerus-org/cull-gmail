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
/// ```rust,no_run
/// // MessageSummary is pub(crate), so this example is for illustration only
/// # struct MessageSummary { id: String, subject: Option<String>, date: Option<String> }
/// # impl MessageSummary {
/// #     fn new(id: &str) -> Self { Self { id: id.to_string(), subject: None, date: None } }
/// #     fn set_subject(&mut self, subject: Option<String>) { self.subject = subject; }
/// #     fn set_date(&mut self, date: Option<String>) { self.date = date; }
/// #     fn subject(&self) -> &str { self.subject.as_deref().unwrap_or("*** No Subject ***") }
/// #     fn date(&self) -> &str { self.date.as_deref().unwrap_or("*** No Date ***") }
/// #     fn list_date_and_subject(&self) -> String { format!("Date: {}, Subject: {}", self.date(), self.subject()) }
/// # }
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
    /// ```rust,no_run
    /// # struct MessageSummary(String);
    /// # impl MessageSummary {
    /// #     fn new(id: &str) -> Self { Self(id.to_string()) }
    /// #     fn id(&self) -> &str { &self.0 }
    /// # }
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
    /// ```rust,no_run
    /// # fn example() { }
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
    /// ```rust,no_run
    /// # fn example() { }
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
    /// ```rust,no_run
    /// # fn example() { }
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
    /// ```rust,no_run
    /// # fn example() { }
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
    /// ```rust,no_run
    /// # fn example() { }
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
    /// ```rust,no_run
    /// # fn example() { }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_summary_new() {
        let summary = MessageSummary::new("test_message_id");
        assert_eq!(summary.id(), "test_message_id");
        assert_eq!(summary.subject(), "*** No Subject for Message ***");
        assert_eq!(summary.date(), "*** No Date for Message ***");
    }

    #[test]
    fn test_message_summary_set_subject() {
        let mut summary = MessageSummary::new("test_id");
        
        // Test setting a subject
        summary.set_subject(Some("Test Subject".to_string()));
        assert_eq!(summary.subject(), "Test Subject");
        
        // Test setting subject to None
        summary.set_subject(None);
        assert_eq!(summary.subject(), "*** No Subject for Message ***");
        
        // Test empty subject
        summary.set_subject(Some("".to_string()));
        assert_eq!(summary.subject(), "");
    }

    #[test]
    fn test_message_summary_set_date() {
        let mut summary = MessageSummary::new("test_id");
        
        // Test setting a date
        summary.set_date(Some("2023-12-25 10:30:00".to_string()));
        assert_eq!(summary.date(), "2023-12-25 10:30:00");
        
        // Test setting date to None
        summary.set_date(None);
        assert_eq!(summary.date(), "*** No Date for Message ***");
        
        // Test empty date
        summary.set_date(Some("".to_string()));
        assert_eq!(summary.date(), "");
    }

    #[test]
    fn test_message_summary_list_date_and_subject_valid() {
        let mut summary = MessageSummary::new("test_id");
        
        // Set up a realistic date and subject
        summary.set_date(Some("2023-12-25 10:30:00 GMT".to_string()));
        summary.set_subject(Some("This is a very long subject that should be elided".to_string()));
        
        let display = summary.list_date_and_subject();
        
        // The method extracts characters 5-16 from date and elides subject to 24 chars
        // "2023-12-25 10:30:00 GMT" -> chars 5-16 would be "2-25 10:30"
        assert!(display.contains("2-25 10:30"));
        assert!(display.contains(":"));
        assert!(display.len() <= 40); // Should be reasonably short for display
    }

    #[test]
    fn test_message_summary_list_date_and_subject_missing_fields() {
        let mut summary = MessageSummary::new("test_id");
        
        // Test with missing date
        summary.set_subject(Some("Test Subject".to_string()));
        let result = summary.list_date_and_subject();
        assert_eq!(result, "***invalid date or subject***");
        
        // Test with missing subject
        let mut summary2 = MessageSummary::new("test_id");
        summary2.set_date(Some("2023-12-25 10:30:00".to_string()));
        let result2 = summary2.list_date_and_subject();
        assert_eq!(result2, "***invalid date or subject***");
        
        // Test with both missing
        let summary3 = MessageSummary::new("test_id");
        let result3 = summary3.list_date_and_subject();
        assert_eq!(result3, "***invalid date or subject***");
    }

    #[test]
    fn test_message_summary_clone() {
        let mut original = MessageSummary::new("original_id");
        original.set_subject(Some("Original Subject".to_string()));
        original.set_date(Some("2023-12-25 10:30:00".to_string()));
        
        let cloned = original.clone();
        
        assert_eq!(original.id(), cloned.id());
        assert_eq!(original.subject(), cloned.subject());
        assert_eq!(original.date(), cloned.date());
    }

    #[test]
    fn test_message_summary_debug() {
        let mut summary = MessageSummary::new("debug_test_id");
        summary.set_subject(Some("Debug Subject".to_string()));
        summary.set_date(Some("2023-12-25".to_string()));
        
        let debug_str = format!("{:?}", summary);
        
        // Verify the debug output contains expected fields
        assert!(debug_str.contains("MessageSummary"));
        assert!(debug_str.contains("debug_test_id"));
        assert!(debug_str.contains("Debug Subject"));
        assert!(debug_str.contains("2023-12-25"));
    }

    #[test]
    fn test_message_summary_unicode_handling() {
        let mut summary = MessageSummary::new("unicode_test");
        
        // Test with Unicode characters in subject and date
        summary.set_subject(Some("📧 Important émails with 中文字符".to_string()));
        summary.set_date(Some("2023-12-25 10:30:00 UTC+8 🕒".to_string()));
        
        assert_eq!(summary.subject(), "📧 Important émails with 中文字符");
        assert_eq!(summary.date(), "2023-12-25 10:30:00 UTC+8 🕒");
        
        // Ensure list formatting doesn't panic with Unicode
        let display = summary.list_date_and_subject();
        assert!(!display.is_empty());
    }

    #[test]
    fn test_message_summary_edge_cases() {
        let test_cases = vec![
            ("", "Empty ID"),
            ("a", "Single char ID"),  
            ("very_long_message_id_that_exceeds_normal_length_expectations_123456789", "Very long ID"),
            ("msg-with-dashes", "ID with dashes"),
            ("msg_with_underscores", "ID with underscores"),
            ("123456789", "Numeric ID"),
        ];
        
        for (id, description) in test_cases {
            let summary = MessageSummary::new(id);
            assert_eq!(summary.id(), id, "Failed for case: {}", description);
        }
    }
}
