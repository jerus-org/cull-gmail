use std::{collections::BTreeSet, fmt};

use chrono::{DateTime, Datelike, Local, TimeDelta, TimeZone};
use serde::{Deserialize, Serialize};

use crate::{MessageAge, Retention, eol_action::EolAction};

/// End of life rules
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
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        } else {
            write!(f, "Complete retention rule not set.")
        }
    }
}

impl EolRule {
    pub(crate) fn new(id: usize) -> Self {
        EolRule {
            id,
            action: EolAction::Trash.to_string(),
            ..Default::default()
        }
    }

    pub(crate) fn set_retention(&mut self, value: Retention) -> &mut Self {
        self.retention = value.age().to_string();
        if value.generate_label() {
            self.add_label(&value.age().label());
        }
        self
    }

    pub(crate) fn retention(&self) -> &str {
        &self.retention
    }

    pub(crate) fn add_label(&mut self, value: &str) -> &mut Self {
        self.labels.insert(value.to_string());
        self
    }

    pub(crate) fn remove_label(&mut self, value: &str) {
        self.labels.remove(value);
    }

    /// Return the id for the rule
    pub fn id(&self) -> usize {
        self.id
    }

    /// List the labels in the rules
    pub fn labels(&self) -> Vec<String> {
        self.labels.iter().map(|i| i.to_string()).collect()
    }

    pub(crate) fn set_action(&mut self, value: &EolAction) -> &mut Self {
        self.action = value.to_string();
        self
    }

    /// Report the action
    pub fn action(&self) -> Option<EolAction> {
        EolAction::parse(&self.action)
    }

    /// Describe the action that will be performed by the rule and its conditions
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
        let count = count.parse::<usize>().unwrap();
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

    pub(crate) fn eol_query(&self) -> Option<String> {
        let today = chrono::Local::now();
        self.calculate_for_date(today)
    }

    fn calculate_for_date(&self, today: DateTime<Local>) -> Option<String> {
        let message_age = MessageAge::parse(&self.retention)?;

        let deadline = match message_age {
            MessageAge::Days(c) => {
                let delta = TimeDelta::days(c);
                today.checked_sub_signed(delta).unwrap()
            }
            MessageAge::Weeks(c) => {
                let delta = TimeDelta::weeks(c);
                today.checked_sub_signed(delta).unwrap()
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
                    .unwrap()
            }
            MessageAge::Years(c) => {
                let day = today.day();
                let month = today.month();
                let year = today.year();
                let new_year = year - c as i32;

                Local
                    .with_ymd_and_hms(new_year, month, day, 0, 0, 0)
                    .unwrap()
            }
        };

        Some(format!("before: {}", deadline.format("%Y-%m-%d")))
    }
}

#[cfg(test)]
mod test {
    use chrono::{Local, TimeZone};

    use crate::{MessageAge, Retention, rules::eol_rule::EolRule};

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

        let test_today = Local.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap();
        let query = rule.calculate_for_date(test_today).unwrap();

        assert_eq!("before: 2020-09-15", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_1_month() {
        let rule = build_test_rule(crate::MessageAge::Months(1));

        let test_today = Local.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap();
        let query = rule.calculate_for_date(test_today).unwrap();

        assert_eq!("before: 2025-08-15", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_13_weeks() {
        let rule = build_test_rule(crate::MessageAge::Weeks(13));

        let test_today = Local.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap();
        let query = rule.calculate_for_date(test_today).unwrap();

        assert_eq!("before: 2025-06-16", query);
    }

    #[test]
    fn test_eol_query_for_eol_rule_365_days() {
        let rule = build_test_rule(crate::MessageAge::Days(365));

        let test_today = Local.with_ymd_and_hms(2025, 9, 15, 0, 0, 0).unwrap();
        let query = rule.calculate_for_date(test_today).unwrap();

        assert_eq!("before: 2024-09-15", query);
    }
}
