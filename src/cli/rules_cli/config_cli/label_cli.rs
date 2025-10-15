use clap::{Parser, Subcommand};
use cull_gmail::{Error, Rules};

mod add_cli;
mod list_cli;
mod remove_cli;

use add_cli::AddCli;
use list_cli::ListCli;
use remove_cli::RemoveCli;

#[derive(Debug, Subcommand)]
pub enum LabelCommands {
    /// List the labels associated with a rule
    #[clap(name = "list")]
    List(ListCli),
    /// Add label to rule
    #[clap(name = "add")]
    Add(AddCli),
    /// Remove a label from a
    #[clap(name = "remove", alias = "rm")]
    Remove(RemoveCli),
}

#[derive(Debug, Parser)]
pub struct LabelCli {
    /// Configuration commands
    #[command(subcommand)]
    command: LabelCommands,
}

impl LabelCli {
    pub fn run(&self, config: Rules) -> Result<(), Error> {
        match &self.command {
            LabelCommands::List(list_cli) => list_cli.run(config),
            LabelCommands::Add(add_cli) => add_cli.run(config),
            LabelCommands::Remove(rm_cli) => rm_cli.run(config),
        }
    }
}
