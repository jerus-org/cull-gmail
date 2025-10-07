use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ConfigCli {
    /// Configuration commands
    #[command(subcommand)]
    command: ConfigCommands,
}

impl ConfigCli {
    pub fn run(&self) {
        match self.command {
            ConfigCommands::List => todo!(),
            ConfigCommands::Add => todo!(),
            ConfigCommands::Remove => todo!(),
            ConfigCommands::Update => todo!(),
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
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
