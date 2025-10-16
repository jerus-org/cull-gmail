use clap::Parser;
use cull_gmail::{GmailClient, Result, Rules};

use crate::run_rules;

#[derive(Debug, Parser)]
pub struct RunCli {
    /// Execute the action
    #[clap(short, long, display_order = 1, help_heading = "Action")]
    execute: bool,
    /// Skip any rules that apply the action `trash`
    #[clap(short = 't', long, display_order = 2, help_heading = "Skip Action")]
    skip_trash: bool,
    /// Skip any rules that apply the action `delete`
    #[clap(short = 'd', long, display_order = 3, help_heading = "Skip Action")]
    skip_delete: bool,
}

impl RunCli {
    pub async fn run(&self, client: &mut GmailClient, rules: Rules) -> Result<()> {
        run_rules(client, rules, self.execute).await
    }
}
