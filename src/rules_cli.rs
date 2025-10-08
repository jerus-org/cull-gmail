use clap::{Parser, Subcommand};
use cull_gmail::Config;

#[derive(Debug, Parser)]
pub struct RulesCli {
    /// Configuration commands
    #[command(subcommand)]
    command: RulesCommands,
}

impl RulesCli {
    pub fn run(&self, config: Config) {
        match self.command {
            RulesCommands::List => config.list_rules(),
            RulesCommands::Add => todo!(),
            RulesCommands::Remove => todo!(),
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
    Add,
    /// Remove a rule from the config file
    #[clap(name = "remove", alias = "rm")]
    Remove,
    /// Update a rule in the config file
    #[clap(name = "update")]
    Update,
}
