use clap::{Parser, Subcommand};
use cull_gmail::{Config, Error};

mod add_cli;
mod rm_cli;

use add_cli::AddCli;
use rm_cli::RmCli;

#[derive(Debug, Parser)]
pub struct RulesCli {
    /// Configuration commands
    #[command(subcommand)]
    command: RulesCommands,
}

impl RulesCli {
    pub fn run(&self, config: Config) -> Result<(), Error> {
        match &self.command {
            RulesCommands::List => config.list_rules(),
            RulesCommands::Add(add_cli) => add_cli.run(config),
            RulesCommands::Remove(rm_cli) => rm_cli.run(config),
            RulesCommands::Update => todo!(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum RulesCommands {
    /// List the rules configured and saved in the config file
    #[clap(name = "list")]
    List,
    /// Add a rules to the config file
    #[clap(name = "add")]
    Add(AddCli),
    /// Remove a rule from the config file
    #[clap(name = "remove", alias = "rm")]
    Remove(RmCli),
    /// Update a rule in the config file
    #[clap(name = "update")]
    Update,
}
