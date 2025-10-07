use std::fmt::Display;

/// Message age
#[derive(Debug)]
pub enum MessageAge {
    /// Number of days to retain the message
    Days(usize),
    /// Number of weeks to retain the message
    Weeks(usize),
    /// Number of months to retain the message
    Months(usize),
    /// Number of years to retain the message
    Years(usize),
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
    pub(crate) fn label(&self) -> String {
        match self {
            MessageAge::Days(v) => format!("retention/{v}-days"),
            MessageAge::Weeks(v) => format!("retention/:{v}-weeks"),
            MessageAge::Months(v) => format!("retention/:{v}-months"),
            MessageAge::Years(v) => format!("retention/:{v}-years"),
        }
    }
}
