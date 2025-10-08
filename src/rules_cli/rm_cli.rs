use clap::Parser;
use cull_gmail::{Config, Error};

#[derive(Debug, Parser)]
pub struct RmCli {
    /// Id of the rule to remove
    #[clap(short, long)]
    id: Option<usize>,
    /// Label in the rule to remove (the rule will be removed)
    #[clap(short, long)]
    label: Option<String>,
}

impl RmCli {
    pub fn run(&self, mut config: Config) -> Result<(), Error> {
        if self.id.is_none() && self.label.is_none() {
            return Err(Error::NoRuleSelector);
        }

        if let Some(id) = self.id {
            config.remove_rule_by_id(id)?;
        }

        if let Some(label) = &self.label {
            config.remove_rule_by_label(label)?;
        }

        Ok(())
    }
}
