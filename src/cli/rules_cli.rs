use clap::Parser;
use cull_gmail::{Config, EolAction, GmailClient, Result, RuleProcessor};

#[derive(Debug, Parser)]
pub struct RulesCli {
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

impl RulesCli {
    pub async fn run(&self, client: &mut GmailClient, config: Config) -> Result<()> {
        let rules = config.get_rules_by_label();

        for label in config.labels() {
            let Some(rule) = rules.get(&label) else {
                log::warn!("no rule found for label `{label}`");
                continue;
            };

            log::info!("Executing rule `#{}` for label `{label}`", rule.describe());
            client.set_rule(rule.clone());
            client.set_execute(self.execute);
            client.find_rule_and_messages_for_label(&label).await?;
            let Some(action) = client.action() else {
                log::warn!("no valid action specified for rule #{}", rule.id());
                continue;
            };

            if self.execute {
                match action {
                    EolAction::Trash => {
                        log::info!("***executing trash messages***");
                        if client.batch_trash().await.is_err() {
                            log::warn!("Move to trash failed for label `{label}`");
                            continue;
                        }
                    }
                    EolAction::Delete => {
                        log::info!("***executing final delete messages***");
                        if client.batch_delete().await.is_err() {
                            log::warn!("Delete failed for label `{label}`");
                            continue;
                        }
                    }
                }
            } else {
                log::warn!("Execution stopped for dry run");
            }
        }

        Ok(())
    }
}
