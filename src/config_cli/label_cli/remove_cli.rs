use clap::Parser;

use cull_gmail::{Config, Result};

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
    pub fn run(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
