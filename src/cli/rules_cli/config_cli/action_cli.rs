use clap::{Parser, ValueEnum};
use cull_gmail::{Rules, EolAction, Error, Result};

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum Action {
    /// Set the action to trash
    #[clap(name = "trash")]
    Trash,
    /// Set the action to
    #[clap(name = "add")]
    Delete,
}

#[derive(Debug, Parser)]
pub struct ActionCli {
    /// Id of the rule on which action applies
    #[clap(short, long)]
    id: usize,
    /// Configuration commands
    #[command(subcommand)]
    action: Action,
}

impl ActionCli {
    pub fn run(&self, mut config: Rules) -> Result<()> {
        if config.get_rule(self.id).is_none() {
            return Err(Error::RuleNotFound(self.id));
        }

        match self.action {
            Action::Trash => config.set_action_on_rule(self.id, &EolAction::Trash),
            Action::Delete => config.set_action_on_rule(self.id, &EolAction::Trash),
        }
    }
}
