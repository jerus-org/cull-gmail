use clap::Parser;

use cull_gmail::{Config, Result};

#[derive(Debug, Parser)]
pub struct ListCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
}

impl ListCli {
    pub fn run(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
