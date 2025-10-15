use clap::Parser;

use cull_gmail::{Rules, Error, Result};

#[derive(Debug, Parser)]
pub struct RemoveCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
    /// Label to remove from the rule
    #[clap(short, long)]
    label: String,
}

impl RemoveCli {
    pub fn run(&self, mut config: Rules) -> Result<()> {
        if config.get_rule(self.id).is_none() {
            return Err(Error::RuleNotFound(self.id));
        }

        config.remove_label_from_rule(self.id, &self.label)
    }
}
