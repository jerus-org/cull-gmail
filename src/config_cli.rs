use clap::{Parser, Subcommand};

mod action_cli;
mod label_cli;
mod rules_cli;

use action_cli::ActionCli;
use cull_gmail::{Config, Result};
use label_cli::LabelCli;
use rules_cli::RulesCli;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Configure end-of-life rules
    #[clap(name = "rules")]
    Rules(RulesCli),
    /// Add or remove Label from rule
    #[clap(name = "label")]
    Label(LabelCli),
    /// Set action on a specific rule
    #[clap(name = "action")]
    Action(ActionCli),
}

#[derive(Parser, Debug)]
pub struct ConfigCli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Commands,
}

impl ConfigCli {
    pub fn run(&self, config: Config) -> Result<()> {
        match &self.command {
            Commands::Rules(rules_cli) => rules_cli.run(config),
            Commands::Label(label_cli) => label_cli.run(config),
            Commands::Action(action_cli) => action_cli.run(config),
        }
    }
}
