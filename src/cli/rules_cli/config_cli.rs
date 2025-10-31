use clap::{Parser, Subcommand};

mod action_rule_cli;
mod add_rule_cli;
mod label_cli;
mod rm_rule_cli;

use action_rule_cli::ActionRuleCli;
use cull_gmail::{Result, Rules};
use label_cli::LabelCli;

#[derive(Subcommand, Debug)]
enum SubCmds {
    /// List the rules configured and saved in the config file
    #[clap(name = "list")]
    List,
    /// Add a rules to the config file
    #[clap(name = "add-rule")]
    Add(add_rule_cli::AddRuleCli),
    /// Remove a rule from the config file
    #[clap(name = "remove-rule", alias = "rm")]
    Remove(rm_rule_cli::RmRuleCli),
    /// Add or remove Label from rule
    #[clap(name = "label")]
    Label(LabelCli),
    /// Set action on a specific rule
    #[clap(name = "set-action-on-rule")]
    Action(ActionRuleCli),
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
