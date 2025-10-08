use std::fmt;

use clap::{Parser, ValueEnum};
use cull_gmail::{Config, Error, MessageAge, Retention};

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Period {
    Days,
    Weeks,
    Months,
    Years,
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Period::Days => write!(f, "days"),
            Period::Weeks => write!(f, "weeks"),
            Period::Months => write!(f, "months"),
            Period::Years => write!(f, "years"),
        }
    }
}

#[derive(Debug, Parser)]
pub struct AddCli {
    /// Period for the rule
    #[arg(short, long)]
    period: Period,
    /// Count of the period
    #[arg(short, long, default_value = "1")]
    count: usize,
    /// Optional specific label; if not specified one will be generated
    #[arg(short, long)]
    label: Option<String>,
}

impl AddCli {
    pub fn run(&self, mut config: Config) -> Result<(), Error> {
        let generate = self.label.is_none();
        let message_age = MessageAge::new(self.period.to_string().as_str(), self.count);
        let retention = Retention::new(message_age, generate);

        config.add_rule(retention, self.label.as_ref());
        config.save()
    }
}
