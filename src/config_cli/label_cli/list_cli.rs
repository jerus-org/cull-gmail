use clap::Parser;

use cull_gmail::{Config, Error, Result};

#[derive(Debug, Parser)]
pub struct ListCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
}

impl ListCli {
    pub fn run(&self, config: Config) -> Result<()> {
        let Some(rule) = config.get_rule(self.id) else {
            return Err(Error::RuleNotFound(self.id));
        };

        for label in rule.labels() {
            log::info!("Label in rule: `{label}`");
        }

        Ok(())
    }
}
