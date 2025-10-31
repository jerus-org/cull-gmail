use clap::{Parser, Subcommand};

mod action_rule_cli;
mod add_label_cli;
mod add_rule_cli;
mod list_label_cli;
mod remove_label_cli;
mod rm_rule_cli;

use action_rule_cli::ActionRuleCli;
use add_label_cli::AddLabelCli;
use cull_gmail::{Result, Rules};
use list_label_cli::ListLabelCli;
use remove_label_cli::RemoveLabelCli;

#[derive(Subcommand, Debug)]
enum SubCmds {
    /// List the rules configured and saved in the config file
    // #[clap(name = "list-rules", subcommand_help_heading = "Rules")]
    #[clap(name = "list-rules")]
    ListRules,
    /// Add a rules to the config file
    // #[clap(name = "add-rule", subcommand_help_heading = "Rules")]
    #[clap(name = "add-rule")]
    AddRule(add_rule_cli::AddRuleCli),
    /// Remove a rule from the config file
    // #[clap(
    //     name = "remove-rule",
    //     alias = "rm-rule",
    //     subcommand_help_heading = "Rules"
    // )]
    #[clap(name = "remove-rule", alias = "rm-rule")]
    RemoveRule(rm_rule_cli::RmRuleCli),
    // #[clap(name = "set-action-on-rule", subcommand_help_heading = "Rules")]
    #[clap(name = "set-action-on-rule")]
    ActionRule(ActionRuleCli),
    /// List the labels associated with a rule
    // #[clap(name = "list-labels", subcommand_help_heading = "Label")]
    #[clap(name = "list-labels")]
    List(ListLabelCli),
    /// Add label to rule
    // #[clap(name = "add-label", subcommand_help_heading = "Label")]
    #[clap(name = "add-label")]
    Add(AddLabelCli),
    /// Remove a label from a
    // #[clap(
    //     name = "remove-label",
    //     alias = "rm-label",
    //     subcommand_help_heading = "Label"
    // )]
    #[clap(name = "remove-label", alias = "rm-label")]
    Remove(RemoveLabelCli),
}

#[derive(Parser, Debug)]
pub struct ConfigCli {
    #[command(subcommand)]
    sub_command: SubCmds,
}

impl ConfigCli {
    pub fn run(&self, rules: Rules) -> Result<()> {
        match &self.sub_command {
            SubCmds::ActionRule(action_cli) => action_cli.run(rules),
            SubCmds::ListRules => rules.list_rules(),
            SubCmds::AddRule(add_cli) => add_cli.run(rules),
            SubCmds::RemoveRule(rm_cli) => rm_cli.run(rules),
            SubCmds::List(list_cli) => list_cli.run(rules),
            SubCmds::Add(add_cli) => add_cli.run(rules),
            SubCmds::Remove(rm_cli) => rm_cli.run(rules),
        }
    }
}
