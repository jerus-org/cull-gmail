use std::fmt::Display;

/// Message age
#[derive(Debug)]
pub enum MessageAge {
    /// Number of days to retain the message
    Days(i64),
    /// Number of weeks to retain the message
    Weeks(i64),
    /// Number of months to retain the message
    Months(i64),
    /// Number of years to retain the message
    Years(i64),
}

impl Display for MessageAge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageAge::Days(v) => write!(f, "d:{v}"),
            MessageAge::Weeks(v) => write!(f, "w:{v}"),
            MessageAge::Months(v) => write!(f, "m:{v}"),
            MessageAge::Years(v) => write!(f, "y:{v}"),
        }
    }
}

impl MessageAge {
    /// Create a new MessageAge enum
    pub fn new(period: &str, count: i64) -> Self {
        match period.to_lowercase().as_str() {
            "days" => MessageAge::Days(count),
            "weeks" => MessageAge::Weeks(count),
            "months" => MessageAge::Months(count),
            "years" => MessageAge::Years(count),
            _ => unreachable!(),
        }
    }

    pub(crate) fn parse(str: &str) -> Option<MessageAge> {
        let period = str.chars().nth(0).unwrap_or('x');
        let count = str[2..].to_string().parse::<i64>().unwrap_or(0);

        match period {
            'd' => Some(MessageAge::Days(count)),
            'w' => Some(MessageAge::Weeks(count)),
            'm' => Some(MessageAge::Months(count)),
            'y' => Some(MessageAge::Years(count)),
            _ => None,
        }
    }

    pub(crate) fn label(&self) -> String {
        match self {
            MessageAge::Days(v) => format!("retention/{v}-days"),
            MessageAge::Weeks(v) => format!("retention/{v}-weeks"),
            MessageAge::Months(v) => format!("retention/{v}-months"),
            MessageAge::Years(v) => format!("retention/{v}-years"),
        }
    }
}
