use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{Retention, eol_cmd::EolCmd};

/// End of life rules
#[derive(Debug, Serialize, Deserialize, Default)]
pub(crate) struct EolRule {
    id: usize,
    retention: String,
    labels: Vec<String>,
    query: Option<String>,
    command: String,
}

impl fmt::Display for EolRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.retention.is_empty() {
            let count = &self.retention[2..];
            let count = count.parse::<usize>().unwrap();
            let period = match self.retention.chars().nth(0) {
                Some('d') => {
                    if count == 1 {
                        "day"
                    } else {
                        "days"
                    }
                }
                Some('w') => {
                    if count == 1 {
                        "week"
                    } else {
                        "weeks"
                    }
                }
                Some('m') => {
                    if count == 1 {
                        "month"
                    } else {
                        "months"
                    }
                }
                Some('y') => {
                    if count == 1 {
                        "year"
                    } else {
                        "years"
                    }
                }
                Some(_) => unreachable!(),
                None => unreachable!(),
            };
            write!(
                f,
                "Rule #{} is active on {} and applies when the message is {count} {period} old.",
                self.id,
                self.labels.join(", ")
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
            command: EolCmd::Trash.to_string(),
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

    pub(crate) fn add_label(&mut self, value: &str) -> &mut Self {
        self.labels.push(value.to_string());
        self
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod test {
    use crate::{Retention, config::eol_rule::EolRule};

    #[test]
    fn test_display_for_eol_rule_5_years() {
        let retention = Retention::new(crate::MessageAge::Years(5), true);
        let mut rule = EolRule::new(1);
        rule.set_retention(retention);
        assert_eq!(
            "Rule #1 is active on retention/5-years and applies when the message is 5 years old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_1_month() {
        let retention = Retention::new(crate::MessageAge::Months(1), true);
        let mut rule = EolRule::new(2);
        rule.set_retention(retention);
        assert_eq!(
            "Rule #2 is active on retention/1-months and applies when the message is 1 month old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_13_weeks() {
        let retention = Retention::new(crate::MessageAge::Weeks(13), true);
        let mut rule = EolRule::new(3);
        rule.set_retention(retention);
        assert_eq!(
            "Rule #3 is active on retention/13-weeks and applies when the message is 13 weeks old."
                .to_string(),
            rule.to_string()
        );
    }

    #[test]
    fn test_display_for_eol_rule_365_days() {
        let retention = Retention::new(crate::MessageAge::Days(365), true);
        let mut rule = EolRule::new(4);
        rule.set_retention(retention);
        assert_eq!(
            "Rule #4 is active on retention/365-days and applies when the message is 365 days old."
                .to_string(),
            rule.to_string()
        );
    }
}
