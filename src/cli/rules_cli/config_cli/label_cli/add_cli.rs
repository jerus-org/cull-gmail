use clap::Parser;

use cull_gmail::{Rules, Error, Result};

#[derive(Debug, Parser)]
pub struct AddCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
    /// Label to add to the rule
    #[clap(short, long)]
    label: String,
}

impl AddCli {
    pub fn run(&self, mut config: Rules) -> Result<()> {
        if config.get_rule(self.id).is_none() {
            return Err(Error::RuleNotFound(self.id));
        }

        config.add_label_to_rule(self.id, &self.label)
    }
}
