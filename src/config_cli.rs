use clap::{Parser, Subcommand};

mod label_cli;
mod rules_cli;

use cull_gmail::{Config, Result};
use label_cli::LabelCli;
use rules_cli::RulesCli;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configure end-of-life rules
    #[clap(name = "rules")]
    Rules(RulesCli),
    /// Add ore remove Label from rule
    #[clap(name = "label")]
    Label(LabelCli),
}

#[derive(Parser, Debug)]
pub struct ConfigCli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Option<Commands>,
}

impl ConfigCli {
    pub fn run(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
