mod message_age;

pub use message_age::MessageAge;

/// Retention policy configuration for email messages.
///
/// A retention policy defines how old messages should be before they are subject
/// to retention actions (trash/delete), and whether a label should be automatically
/// generated to categorize messages matching this policy.
///
/// # Examples
///
/// ```
/// use cull_gmail::{Retention, MessageAge};
///
/// // Create a retention policy for messages older than 6 months
/// let policy = Retention::new(MessageAge::Months(6), true);
///
/// // Create a retention policy without auto-generated labels
/// let policy = Retention::new(MessageAge::Years(1), false);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Retention {
    age: MessageAge,
    generate_label: bool,
}

impl Default for Retention {
    fn default() -> Self {
        Self {
            age: MessageAge::Years(5),
            generate_label: true,
        }
    }
}

impl Retention {
    /// Create a new retention policy.
    ///
    /// # Arguments
    ///
    /// * `age` - The message age threshold for this retention policy
    /// * `generate_label` - Whether to automatically generate a label for messages matching this policy
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Retention, MessageAge};
    ///
    /// // Policy for messages older than 30 days with auto-generated label
    /// let policy = Retention::new(MessageAge::Days(30), true);
    ///
    /// // Policy for messages older than 1 year without label generation
    /// let policy = Retention::new(MessageAge::Years(1), false);
    /// ```
    pub fn new(age: MessageAge, generate_label: bool) -> Self {
        Retention {
            age,
            generate_label,
        }
    }

    /// Get the message age threshold for this retention policy.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Retention, MessageAge};
    ///
    /// let policy = Retention::new(MessageAge::Days(30), true);
    /// assert_eq!(policy.age(), &MessageAge::Days(30));
    /// ```
    #[must_use]
    pub fn age(&self) -> &MessageAge {
        &self.age
    }

    /// Check if this retention policy should generate automatic labels.
    ///
    /// When `true`, messages matching this retention policy will be automatically
    /// tagged with a generated label based on the age specification.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Retention, MessageAge};
    ///
    /// let policy = Retention::new(MessageAge::Days(30), true);
    /// assert_eq!(policy.generate_label(), true);
    ///
    /// let policy = Retention::new(MessageAge::Days(30), false);
    /// assert_eq!(policy.generate_label(), false);
    /// ```
    #[must_use]
    pub fn generate_label(&self) -> bool {
        self.generate_label
    }

    /// Set whether this retention policy should generate automatic labels.
    ///
    /// # Examples
    ///
    /// ```
    /// use cull_gmail::{Retention, MessageAge};
    ///
    /// let mut policy = Retention::new(MessageAge::Days(30), false);
    /// policy.set_generate_label(true);
    /// assert_eq!(policy.generate_label(), true);
    /// ```
    pub fn set_generate_label(&mut self, generate_label: bool) {
        self.generate_label = generate_label;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retention_new() {
        let age = MessageAge::Days(30);
        let retention = Retention::new(age.clone(), true);

        assert_eq!(retention.age(), &age);
        assert!(retention.generate_label());
    }

    #[test]
    fn test_retention_new_no_label() {
        let age = MessageAge::Years(1);
        let retention = Retention::new(age.clone(), false);

        assert_eq!(retention.age(), &age);
        assert!(!retention.generate_label());
    }

    #[test]
    fn test_retention_set_generate_label() {
        let age = MessageAge::Months(6);
        let mut retention = Retention::new(age.clone(), false);

        assert!(!retention.generate_label());

        retention.set_generate_label(true);
        assert!(retention.generate_label());

        retention.set_generate_label(false);
        assert!(!retention.generate_label());
    }

    #[test]
    fn test_retention_clone() {
        let age = MessageAge::Weeks(2);
        let original = Retention::new(age.clone(), true);
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.age(), cloned.age());
        assert_eq!(original.generate_label(), cloned.generate_label());
    }

    #[test]
    fn test_retention_equality() {
        let age1 = MessageAge::Days(30);
        let age2 = MessageAge::Days(30);
        let age3 = MessageAge::Days(31);

        let retention1 = Retention::new(age1, true);
        let retention2 = Retention::new(age2, true);
        let retention3 = Retention::new(age3, true);
        let retention4 = Retention::new(MessageAge::Days(30), false);

        assert_eq!(retention1, retention2);
        assert_ne!(retention1, retention3); // different age
        assert_ne!(retention1, retention4); // different generate_label
    }

    #[test]
    fn test_retention_default() {
        let default = Retention::default();

        assert_eq!(default.age(), &MessageAge::Years(5));
        assert!(default.generate_label());
    }

    #[test]
    fn test_retention_with_different_age_types() {
        let retention_days = Retention::new(MessageAge::Days(90), true);
        let retention_weeks = Retention::new(MessageAge::Weeks(12), false);
        let retention_months = Retention::new(MessageAge::Months(3), true);
        let retention_years = Retention::new(MessageAge::Years(1), false);

        assert_eq!(retention_days.age().period_type(), "days");
        assert_eq!(retention_weeks.age().period_type(), "weeks");
        assert_eq!(retention_months.age().period_type(), "months");
        assert_eq!(retention_years.age().period_type(), "years");

        assert!(retention_days.generate_label());
        assert!(!retention_weeks.generate_label());
        assert!(retention_months.generate_label());
        assert!(!retention_years.generate_label());
    }

    #[test]
    fn test_retention_debug() {
        let retention = Retention::new(MessageAge::Days(30), true);
        let debug_str = format!("{:?}", retention);

        assert!(debug_str.contains("Retention"));
        assert!(debug_str.contains("Days(30)"));
        assert!(debug_str.contains("true"));
    }
}
