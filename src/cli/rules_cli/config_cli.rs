use clap::{Parser, Subcommand};

mod action_cli;
mod label_cli;
mod rules_cli;

use action_cli::ActionCli;
use cull_gmail::{Result, Rules};
use label_cli::LabelCli;
use rules_cli::RulesCli;

#[derive(Subcommand, Debug)]
enum SubCmds {
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
    sub_command: SubCmds,
}

impl ConfigCli {
    pub fn run(&self, config: Rules) -> Result<()> {
        match &self.sub_command {
            SubCmds::Rules(rules_cli) => rules_cli.run(config),
            SubCmds::Label(label_cli) => label_cli.run(config),
            SubCmds::Action(action_cli) => action_cli.run(config),
        }
    }
}
