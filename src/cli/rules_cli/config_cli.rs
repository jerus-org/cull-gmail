use clap::{Parser, Subcommand};

mod action_cli;
mod add_cli;
mod label_cli;
mod rm_cli;

use action_cli::ActionCli;
use cull_gmail::{Result, Rules};
use label_cli::LabelCli;

#[derive(Subcommand, Debug)]
enum SubCmds {
    /// List the rules configured and saved in the config file
    #[clap(name = "list")]
    List,
    /// Add a rules to the config file
    #[clap(name = "add")]
    Add(add_cli::AddCli),
    /// Remove a rule from the config file
    #[clap(name = "remove", alias = "rm")]
    Remove(rm_cli::RmCli),
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
    sub_command: SubCmds,
}

impl ConfigCli {
    pub fn run(&self, rules: Rules) -> Result<()> {
        match &self.sub_command {
            SubCmds::Label(label_cli) => label_cli.run(rules),
            SubCmds::Action(action_cli) => action_cli.run(rules),
            SubCmds::List => rules.list_rules(),
            SubCmds::Add(add_cli) => add_cli.run(rules),
            SubCmds::Remove(rm_cli) => rm_cli.run(rules),
        }
    }
}
