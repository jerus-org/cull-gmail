//! End-of-life (EOL) rule implementation.
//!
//! This module provides the [`EolRule`] struct which defines rules for automatically
//! processing Gmail messages based on their age. Rules can be configured to either
//! move messages to trash or permanently delete them after a specified retention period.
//!
//! # Usage
//!
//! ```ignore
//! use cull_gmail::{Retention, MessageAge, EolAction};
//! use cull_gmail::rules::eol_rule::EolRule;
//!
//! // Create a new rule to delete messages older than 1 year
//! let mut rule = EolRule::new(1);
//! let retention = Retention::new(MessageAge::Years(1), true);
//! rule.set_retention(retention);
//! rule.set_action(&EolAction::Delete);
//!
//! // Add labels that this rule applies to
//! rule.add_label("old-emails");
//!
//! println!("Rule description: {}", rule.describe());
//! ```

use std::{collections::BTreeSet, fmt};

use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone};
use serde::{Deserialize, Serialize};

use crate::{MessageAge, Retention, eol_action::EolAction};

/// A rule that defines end-of-life processing for Gmail messages.
///
/// An `EolRule` specifies conditions under which messages should be processed
/// (moved to trash or deleted) based on their age and optional label filters.
/// Each rule has a unique identifier and can be configured with retention periods,
/// target labels, and actions to perform.
///
/// # Examples
///
/// Creating a basic rule:
///
/// ```ignore
/// # use cull_gmail::rules::eol_rule::EolRule;
/// let rule = EolRule::new(42);
/// assert_eq!(rule.id(), 42);
/// assert!(rule.labels().is_empty());
/// ```
///
/// # Serialization
///
/// Rules can be serialized to and from TOML/JSON using serde.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct EolRule {
    id: usize,
    retention: String,
    labels: BTreeSet<String>,
    query: Option<String>,
    action: String,
}

impl fmt::Display for EolRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.retention.is_empty() {
            let (action, count, period) = self.get_action_period_count_strings();

            write!(
                f,
                "Rule #{} is active on `{}` to {action} if it is more than {count} {period} old.",
                self.id,
                self.labels
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            write!(f, "Complete retention rule not set.")
        }
    }
}

impl EolRule {
    /// Creates a new end-of-life rule with the specified unique identifier.
    ///
    /// The rule is created with default settings:
    /// - Action: Move to trash (not delete)
    /// - No retention period set
    /// - No labels specified
    /// - No custom query
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for this rule
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::rules::eol_rule::EolRule;
    /// let rule = EolRule::new(1);
    /// assert_eq!(rule.id(), 1);
    /// assert!(rule.labels().is_empty());
    /// ```
    pub(crate) fn new(id: usize) -> Self {
        EolRule {
            id,
            action: EolAction::Trash.to_string(),
            ..Default::default()
        }
    }

    /// Sets the retention period for this rule.
    ///
    /// The retention period determines how old messages must be before this rule
    /// applies to them. If the retention is configured to generate labels, the
    /// appropriate label will be automatically added to this rule.
    ///
    /// # Arguments
    ///
    /// * `value` - The retention configuration specifying age and label generation
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, Retention, MessageAge};
    /// let mut rule = EolRule::new(1);
    /// let retention = Retention::new(MessageAge::Months(6), true);
    /// rule.set_retention(retention);
    ///
    /// assert_eq!(rule.retention(), "m:6");
    /// assert!(rule.labels().contains(&"retention/6-months".to_string()));
    /// ```
    pub(crate) fn set_retention(&mut self, value: Retention) -> &mut Self {
        self.retention = value.age().to_string();
        if value.generate_label() {
            self.add_label(&value.age().label());
        }
        self
    }

    /// Returns the retention period string for this rule.
    ///
    /// The retention string follows the format used by [`MessageAge`],
    /// such as "d:30" for 30 days or "y:1" for 1 year.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, Retention, MessageAge};
    /// let mut rule = EolRule::new(1);
    /// let retention = Retention::new(MessageAge::Days(90), false);
    /// rule.set_retention(retention);
    ///
    /// assert_eq!(rule.retention(), "d:90");
    /// ```
    pub(crate) fn retention(&self) -> &str {
        &self.retention
    }

    /// Adds a label that this rule should apply to.
    ///
    /// Labels are used to filter which messages this rule processes. Messages
    /// must have one of the rule's labels to be affected by the rule.
    /// Duplicate labels are ignored.
    ///
    /// # Arguments
    ///
    /// * `value` - The label name to add
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::rules::eol_rule::EolRule;
    /// let mut rule = EolRule::new(1);
    /// rule.add_label("newsletter")
    ///     .add_label("promotions");
    ///
    /// let labels = rule.labels();
    /// assert!(labels.contains(&"newsletter".to_string()));
    /// assert!(labels.contains(&"promotions".to_string()));
    /// assert_eq!(labels.len(), 2);
    /// ```
    pub(crate) fn add_label(&mut self, value: &str) -> &mut Self {
        self.labels.insert(value.to_string());
        self
    }

    /// Removes a label from this rule.
    ///
    /// If the label is not present, this operation does nothing.
    ///
    /// # Arguments
    ///
    /// * `value` - The label name to remove
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::rules::eol_rule::EolRule;
    /// let mut rule = EolRule::new(1);
    /// rule.add_label("temp-label");
    /// assert!(rule.labels().contains(&"temp-label".to_string()));
    ///
    /// rule.remove_label("temp-label");
    /// assert!(!rule.labels().contains(&"temp-label".to_string()));
    /// ```
    pub(crate) fn remove_label(&mut self, value: &str) {
        self.labels.remove(value);
    }

    /// Returns the unique identifier for this rule.
    ///
    /// Each rule has a unique ID that distinguishes it from other rules
    /// in the same rule set.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::rules::eol_rule::EolRule;
    /// let rule = EolRule::new(42);
    /// assert_eq!(rule.id(), 42);
    /// ```
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns a list of all labels that this rule applies to.
    ///
    /// Labels determine which messages this rule will process. Only messages
    /// with one of these labels will be affected by the rule.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::rules::eol_rule::EolRule;
    /// let mut rule = EolRule::new(1);
    /// rule.add_label("spam").add_label("newsletter");
    ///
    /// let labels = rule.labels();
    /// assert_eq!(labels.len(), 2);
    /// assert!(labels.contains(&"spam".to_string()));
    /// assert!(labels.contains(&"newsletter".to_string()));
    /// ```
    pub fn labels(&self) -> Vec<String> {
        self.labels.iter().cloned().collect()
    }

    /// Sets the action to perform when this rule matches messages.
    ///
    /// The action determines what happens to messages that match this rule's
    /// criteria (age and labels).
    ///
    /// # Arguments
    ///
    /// * `value` - The action to perform (Trash or Delete)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, EolAction};
    /// let mut rule = EolRule::new(1);
    /// rule.set_action(&EolAction::Delete);
    ///
    /// assert_eq!(rule.action(), Some(EolAction::Delete));
    /// ```
    pub(crate) fn set_action(&mut self, value: &EolAction) -> &mut Self {
        self.action = value.to_string();
        self
    }

    /// Returns the action that will be performed by this rule.
    ///
    /// The action determines what happens to messages that match this rule:
    /// - `Trash`: Move messages to the trash folder
    /// - `Delete`: Permanently delete messages
    ///
    /// Returns `None` if the action string cannot be parsed (should not happen
    /// with properly constructed rules).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, EolAction};
    /// let mut rule = EolRule::new(1);
    /// rule.set_action(&EolAction::Trash);
    ///
    /// assert_eq!(rule.action(), Some(EolAction::Trash));
    /// ```
    pub fn action(&self) -> Option<EolAction> {
        EolAction::parse(&self.action)
    }

    /// Returns a human-readable description of what this rule does.
    ///
    /// The description includes the rule ID, the action that will be performed,
    /// and the age threshold for messages.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, Retention, MessageAge, EolAction};
    /// let mut rule = EolRule::new(5);
    /// let retention = Retention::new(MessageAge::Months(3), false);
    /// rule.set_retention(retention);
    /// rule.set_action(&EolAction::Delete);
    ///
    /// let description = rule.describe();
    /// assert!(description.contains("Rule #5"));
    /// assert!(description.contains("delete"));
    /// assert!(description.contains("3 months"));
    /// ```
    pub fn describe(&self) -> String {
        let (action, count, period) = self.get_action_period_count_strings();
        format!(
            "Rule #{}, to {action} if it is more than {count} {period} old.",
            self.id,
        )
    }

    /// Describe the action that will be performed by the rule and its conditions
    fn get_action_period_count_strings(&self) -> (String, usize, String) {
        let count = &self.retention[2..];
        let count = count.parse::<usize>().unwrap_or(0); // Default to 0 if parsing fails
        let mut period = match self.retention.chars().nth(0) {
            Some('d') => "day",
            Some('w') => "week",
            Some('m') => "month",
            Some('y') => "year",
            Some(_) => unreachable!(),
            None => unreachable!(),
        }
        .to_string();
        if count > 1 {
            period.push('s');
        }

        let action = match self.action.to_lowercase().as_str() {
            "trash" => "move the message to trash",
            "delete" => "delete the message",
            _ => unreachable!(),
        };

        (action.to_string(), count, period)
    }

    /// Generates a Gmail search query for messages that match this rule's age criteria.
    ///
    /// This method calculates the cut-off date based on the rule's retention period
    /// and returns a Gmail search query string that can be used to find messages
    /// older than the specified threshold.
    ///
    /// Returns `None` if the retention period is not set or cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// # use cull_gmail::{rules::eol_rule::EolRule, Retention, MessageAge};
    /// let mut rule = EolRule::new(1);
    /// let retention = Retention::new(MessageAge::Days(30), false);
    /// rule.set_retention(retention);
    ///
    /// if let Some(query) = rule.eol_query() {
    ///     println!("Gmail query: {}", query);
    ///     // Output will be something like "before: 2024-08-15"
    /// }
    /// ```
    pub(crate) fn eol_query(&self) -> Option<String> {
        let today = chrono::Local::now();
        self.calculate_for_date(today)
    }

    fn calculate_for_date(&self, today: DateTime<Local>) -> Option<String> {
        let message_age = MessageAge::parse(&self.retention)?;
        log::debug!("testing for {message_age}");

        let deadline = match message_age {
            MessageAge::Days(c) => {
                let delta = TimeDelta::days(c);
                log::debug!("delta for change: {delta}");
                let deadline = today.checked_sub_signed(delta)?;
                log::debug!("calculated deadline: {deadline}");
                deadline
            }
            MessageAge::Weeks(c) => {
                let delta = TimeDelta::weeks(c);
                today.checked_sub_signed(delta)?
            }
            MessageAge::Months(c) => {
                let day = today.day();
                let month = today.month();
                let year = today.year();
                let mut years = c as i32 / 12;
                let months = c % 12;
                let mut new_month = month - months as u32;

                if new_month < 1 {
                    years += 1;
                    new_month += 12;
                }

                let new_year = year - years;

                Local
                    .with_ymd_and_hms(new_year, new_month, day, 0, 0, 0)
                    .single()?
            }
            MessageAge::Years(c) => {
                let day = today.day();
                let month = today.month();
                let year = today.year();
                let new_year = year - c as i32;

                Local
                    .with_ymd_and_hms(new_year, month, day, 0, 0, 0)
                    .single()?
            }
        };

        Some(format!("before: {}", deadline.format("%Y-%m-%d")))
    }
}

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};

    use crate::{MessageAge, Retention, rules::eol_rule::EolRule, test_utils::get_test_logger};

    fn build_test_rule(age: MessageAge) -> EolRule {
        let retention = Retention::new(age, true);
        let mut rule = EolRule::new(1);
        rule.set_retention(retention);
        rule
    }

    #[test]
    fn test_display_for_eol_rule_5_years() {
        let rule = build_test_rule(crate::MessageAge::Years(5));

        assert_eq!(
            "Rule #1 is active on `retention/5-years` to move the message to trash if it is more than 5 years old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_1_month() {
        let rule = build_test_rule(crate::MessageAge::Months(1));

        assert_eq!(
            "Rule #1 is active on `retention/1-months` to move the message to trash if it is more than 1 month old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_13_weeks() {
        let rule = build_test_rule(crate::MessageAge::Weeks(13));

        assert_eq!(
            "Rule #1 is active on `retention/13-weeks` to move the message to trash if it is more than 13 weeks old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_365_days() {
        let rule = build_test_rule(crate::MessageAge::Days(365));

        assert_eq!(
            "Rule #1 is active on `retention/365-days` to move the message to trash if it is more than 365 days old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_eol_query_for_eol_rule_5_years() {
        let rule = build_test_rule(crate::MessageAge::Years(5));

        let test_today = Local
            .with_ymd_and_hms(2025, 9, 15, 0, 0, 0)
            .single()
            .unwrap();
        let query = rule
            .calculate_for_date(test_today)
            .expect("Failed to calculate query");

        assert_eq!("before: 2020-09-15", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_1_month() {
        let rule = build_test_rule(crate::MessageAge::Months(1));

        let test_today = Local
            .with_ymd_and_hms(2025, 9, 15, 0, 0, 0)
            .single()
            .unwrap();
        let query = rule
            .calculate_for_date(test_today)
            .expect("Failed to calculate query");

        assert_eq!("before: 2025-08-15", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_13_weeks() {
        let rule = build_test_rule(crate::MessageAge::Weeks(13));

        let test_today = Local
            .with_ymd_and_hms(2025, 9, 15, 0, 0, 0)
            .single()
            .unwrap();
        let query = rule
            .calculate_for_date(test_today)
            .expect("Failed to calculate query");

        assert_eq!("before: 2025-06-16", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_365_days() {
        let rule = build_test_rule(crate::MessageAge::Days(365));

        let test_today = Local
            .with_ymd_and_hms(2025, 9, 15, 0, 0, 0)
            .single()
            .unwrap();
        let query = rule
            .calculate_for_date(test_today)
            .expect("Failed to calculate query");

        assert_eq!("before: 2024-09-15", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_3038_days() {
        get_test_logger();
        let rule = build_test_rule(crate::MessageAge::Days(6580));

        let test_today = Local
            .with_ymd_and_hms(2025, 9, 15, 0, 0, 0)
            .single()
            .unwrap();
        let query = rule
            .calculate_for_date(test_today)
            .expect("Failed to calculate query");

        assert_eq!("before: 2007-09-10", query);
    }
}
