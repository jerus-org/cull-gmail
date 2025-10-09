use clap::Parser;

use cull_gmail::{Config, Result};

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
    pub fn run(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
