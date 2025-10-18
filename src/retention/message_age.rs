use std::fmt::Display;

/// Message age specification for retention policies.
/// 
/// Defines different time periods that can be used to specify how old messages
/// should be before they are subject to retention actions (trash/delete).
/// 
/// # Examples
/// 
/// ```
/// use cull_gmail::MessageAge;
/// 
/// // Create different message age specifications
/// let days = MessageAge::Days(30);
/// let weeks = MessageAge::Weeks(4);
/// let months = MessageAge::Months(6);
/// let years = MessageAge::Years(2);
/// 
/// // Use with retention policy
/// println!("Messages older than {} will be processed", months);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageAge {
    /// Number of days to retain the message
    /// 
    /// # Example
    /// ```
    /// use cull_gmail::MessageAge;
    /// let age = MessageAge::Days(30);
    /// assert_eq!(age.to_string(), "d:30");
    /// ```
    Days(i64),
    /// Number of weeks to retain the message
    /// 
    /// # Example
    /// ```
    /// use cull_gmail::MessageAge;
    /// let age = MessageAge::Weeks(4);
    /// assert_eq!(age.to_string(), "w:4");
    /// ```
    Weeks(i64),
    /// Number of months to retain the message
    /// 
    /// # Example
    /// ```
    /// use cull_gmail::MessageAge;
    /// let age = MessageAge::Months(6);
    /// assert_eq!(age.to_string(), "m:6");
    /// ```
    Months(i64),
    /// Number of years to retain the message
    /// 
    /// # Example
    /// ```
    /// use cull_gmail::MessageAge;
    /// let age = MessageAge::Years(2);
    /// assert_eq!(age.to_string(), "y:2");
    /// ```
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
    /// Create a new MessageAge from a period string and count.
    /// 
    /// # Arguments
    /// 
    /// * `period` - The time period ("days", "weeks", "months", "years")
    /// * `count` - The number of time periods (must be positive)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cull_gmail::MessageAge;
    /// 
    /// let age = MessageAge::new("days", 30).unwrap();
    /// assert_eq!(age, MessageAge::Days(30));
    /// 
    /// let age = MessageAge::new("months", 6).unwrap();
    /// assert_eq!(age, MessageAge::Months(6));
    /// 
    /// // Invalid period returns an error
    /// assert!(MessageAge::new("invalid", 1).is_err());
    /// 
    /// // Negative count returns an error
    /// assert!(MessageAge::new("days", -1).is_err());
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The period string is not recognized
    /// - The count is negative or zero
    pub fn new(period: &str, count: i64) -> Result<Self, String> {
        if count <= 0 {
            return Err(format!("Count must be positive, got: {}", count));
        }
        
        match period.to_lowercase().as_str() {
            "days" => Ok(MessageAge::Days(count)),
            "weeks" => Ok(MessageAge::Weeks(count)),
            "months" => Ok(MessageAge::Months(count)),
            "years" => Ok(MessageAge::Years(count)),
            _ => Err(format!("Unknown period '{}', expected one of: days, weeks, months, years", period)),
        }
    }

    /// Parse a MessageAge from a string representation (e.g., "d:30", "m:6").
    /// 
    /// # Arguments
    /// 
    /// * `s` - String in format "`period:count`" where period is d/w/m/y
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cull_gmail::MessageAge;
    /// 
    /// let age = MessageAge::parse("d:30").unwrap();
    /// assert_eq!(age, MessageAge::Days(30));
    /// 
    /// let age = MessageAge::parse("y:2").unwrap();
    /// assert_eq!(age, MessageAge::Years(2));
    /// 
    /// // Invalid format returns None
    /// assert!(MessageAge::parse("invalid").is_none());
    /// assert!(MessageAge::parse("d").is_none());
    /// ```
    pub fn parse(s: &str) -> Option<MessageAge> {
        if s.len() < 3 || s.chars().nth(1) != Some(':') {
            return None;
        }
        
        let period = s.chars().nth(0)?;
        let count_str = &s[2..];
        let count = count_str.parse::<i64>().ok()?;
        
        if count <= 0 {
            return None;
        }

        match period {
            'd' => Some(MessageAge::Days(count)),
            'w' => Some(MessageAge::Weeks(count)),
            'm' => Some(MessageAge::Months(count)),
            'y' => Some(MessageAge::Years(count)),
            _ => None,
        }
    }

    /// Generate a label string for this message age.
    /// 
    /// This creates a standardized label that can be used to categorize
    /// messages based on their retention period.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cull_gmail::MessageAge;
    /// 
    /// let age = MessageAge::Days(30);
    /// assert_eq!(age.label(), "retention/30-days");
    /// 
    /// let age = MessageAge::Years(1);
    /// assert_eq!(age.label(), "retention/1-years");
    /// ```
    pub fn label(&self) -> String {
        match self {
            MessageAge::Days(v) => format!("retention/{v}-days"),
            MessageAge::Weeks(v) => format!("retention/{v}-weeks"),
            MessageAge::Months(v) => format!("retention/{v}-months"),
            MessageAge::Years(v) => format!("retention/{v}-years"),
        }
    }
    
    /// Get the numeric value of this message age.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cull_gmail::MessageAge;
    /// 
    /// let age = MessageAge::Days(30);
    /// assert_eq!(age.value(), 30);
    /// 
    /// let age = MessageAge::Years(2);
    /// assert_eq!(age.value(), 2);
    /// ```
    pub fn value(&self) -> i64 {
        match self {
            MessageAge::Days(v) | MessageAge::Weeks(v) | MessageAge::Months(v) | MessageAge::Years(v) => *v,
        }
    }
    
    /// Get the period type as a string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cull_gmail::MessageAge;
    /// 
    /// let age = MessageAge::Days(30);
    /// assert_eq!(age.period_type(), "days");
    /// 
    /// let age = MessageAge::Years(2);
    /// assert_eq!(age.period_type(), "years");
    /// ```
    pub fn period_type(&self) -> &'static str {
        match self {
            MessageAge::Days(_) => "days",
            MessageAge::Weeks(_) => "weeks",
            MessageAge::Months(_) => "months",
            MessageAge::Years(_) => "years",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_age_new_valid() {
        // Test valid periods
        assert_eq!(MessageAge::new("days", 30).unwrap(), MessageAge::Days(30));
        assert_eq!(MessageAge::new("weeks", 4).unwrap(), MessageAge::Weeks(4));
        assert_eq!(MessageAge::new("months", 6).unwrap(), MessageAge::Months(6));
        assert_eq!(MessageAge::new("years", 2).unwrap(), MessageAge::Years(2));
        
        // Test case insensitive
        assert_eq!(MessageAge::new("DAYS", 1).unwrap(), MessageAge::Days(1));
        assert_eq!(MessageAge::new("Days", 1).unwrap(), MessageAge::Days(1));
        assert_eq!(MessageAge::new("dAyS", 1).unwrap(), MessageAge::Days(1));
    }

    #[test]
    fn test_message_age_new_invalid_period() {
        assert!(MessageAge::new("invalid", 1).is_err());
        assert!(MessageAge::new("day", 1).is_err());  // singular form
        assert!(MessageAge::new("", 1).is_err());
        
        // Check error messages
        let err = MessageAge::new("invalid", 1).unwrap_err();
        assert!(err.contains("Unknown period 'invalid'"));
    }

    #[test]
    fn test_message_age_new_invalid_count() {
        assert!(MessageAge::new("days", 0).is_err());
        assert!(MessageAge::new("days", -1).is_err());
        assert!(MessageAge::new("days", -100).is_err());
        
        // Check error messages
        let err = MessageAge::new("days", -1).unwrap_err();
        assert!(err.contains("Count must be positive"));
    }

    #[test]
    fn test_message_age_parse_valid() {
        assert_eq!(MessageAge::parse("d:30").unwrap(), MessageAge::Days(30));
        assert_eq!(MessageAge::parse("w:4").unwrap(), MessageAge::Weeks(4));
        assert_eq!(MessageAge::parse("m:6").unwrap(), MessageAge::Months(6));
        assert_eq!(MessageAge::parse("y:2").unwrap(), MessageAge::Years(2));
        
        // Test large numbers
        assert_eq!(MessageAge::parse("d:999").unwrap(), MessageAge::Days(999));
    }

    #[test]
    fn test_message_age_parse_invalid() {
        // Invalid format
        assert!(MessageAge::parse("invalid").is_none());
        assert!(MessageAge::parse("d").is_none());
        assert!(MessageAge::parse("d:").is_none());
        assert!(MessageAge::parse(":30").is_none());
        assert!(MessageAge::parse("x:30").is_none());
        
        // Invalid count
        assert!(MessageAge::parse("d:0").is_none());
        assert!(MessageAge::parse("d:-1").is_none());
        assert!(MessageAge::parse("d:abc").is_none());
        
        // Wrong separator
        assert!(MessageAge::parse("d-30").is_none());
        assert!(MessageAge::parse("d 30").is_none());
    }

    #[test]
    fn test_message_age_display() {
        assert_eq!(MessageAge::Days(30).to_string(), "d:30");
        assert_eq!(MessageAge::Weeks(4).to_string(), "w:4");
        assert_eq!(MessageAge::Months(6).to_string(), "m:6");
        assert_eq!(MessageAge::Years(2).to_string(), "y:2");
    }

    #[test]
    fn test_message_age_label() {
        assert_eq!(MessageAge::Days(30).label(), "retention/30-days");
        assert_eq!(MessageAge::Weeks(4).label(), "retention/4-weeks");
        assert_eq!(MessageAge::Months(6).label(), "retention/6-months");
        assert_eq!(MessageAge::Years(2).label(), "retention/2-years");
    }

    #[test]
    fn test_message_age_value() {
        assert_eq!(MessageAge::Days(30).value(), 30);
        assert_eq!(MessageAge::Weeks(4).value(), 4);
        assert_eq!(MessageAge::Months(6).value(), 6);
        assert_eq!(MessageAge::Years(2).value(), 2);
    }

    #[test]
    fn test_message_age_period_type() {
        assert_eq!(MessageAge::Days(30).period_type(), "days");
        assert_eq!(MessageAge::Weeks(4).period_type(), "weeks");
        assert_eq!(MessageAge::Months(6).period_type(), "months");
        assert_eq!(MessageAge::Years(2).period_type(), "years");
    }

    #[test]
    fn test_message_age_clone() {
        let original = MessageAge::Days(30);
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_message_age_eq() {
        assert_eq!(MessageAge::Days(30), MessageAge::Days(30));
        assert_ne!(MessageAge::Days(30), MessageAge::Days(31));
        assert_ne!(MessageAge::Days(30), MessageAge::Weeks(30));
    }

    #[test]
    fn test_parse_roundtrip() {
        let original = MessageAge::Days(30);
        let serialized = original.to_string();
        let parsed = MessageAge::parse(&serialized).unwrap();
        assert_eq!(original, parsed);
        
        let original = MessageAge::Years(5);
        let serialized = original.to_string();
        let parsed = MessageAge::parse(&serialized).unwrap();
        assert_eq!(original, parsed);
    }
}
