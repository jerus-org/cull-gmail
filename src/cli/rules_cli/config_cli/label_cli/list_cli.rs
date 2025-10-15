use clap::Parser;

use cull_gmail::{Error, Result, Rules};

#[derive(Debug, Parser)]
pub struct ListCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
}

impl ListCli {
    pub fn run(&self, config: Rules) -> Result<()> {
        let Some(rule) = config.get_rule(self.id) else {
            return Err(Error::RuleNotFound(self.id));
        };

        print!("Labels in rule: ");
        for (i, label) in rule.labels().iter().enumerate() {
            if i != 0 {
                print!(", {label}");
            } else {
                print!("`{label}");
            }
        }
        println!("`");
        Ok(())
    }
}
