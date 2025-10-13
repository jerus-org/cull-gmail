use clap::Parser;
use cull_gmail::{Config, EolAction, GmailClient, Processor, Result};

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
    pub async fn run(&self, client: &GmailClient, config: Config) -> Result<()> {
        let rules = config.get_rules_by_label();

        for label in config.labels() {
            let Some(rule) = rules.get(&label) else {
                log::warn!("no rule found for label `{label}`");
                continue;
            };

            log::info!("Executing rule `#{}` for label `{label}`", rule.describe());

            let mut builder = Processor::builder(client, rule);
            let processor = builder.set_execute(self.execute).build();

            let Some(action) = processor.action() else {
                log::warn!("no valid action specified for rule #{}", rule.id());
                continue;
            };

            self.execute_action(processor, action, &label).await;
        }

        Ok(())
    }

    async fn execute_action<'a>(&self, processor: Processor<'a>, action: EolAction, label: &str) {
        match action {
            EolAction::Trash => {
                if !self.skip_trash {
                    log::info!("trashing older messages");
                    match processor.trash_messages(label).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("action failed for label {label} with error {e}");
                        }
                    }
                } else {
                    log::warn!("Rule with `trash` action for label `{label}` skipped.");
                }
            }
            EolAction::Delete => {
                if !self.skip_delete {
                    log::info!("deleting older messages");
                    match processor.delete_messages(label).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("action failed for label {label} with error {e}");
                        }
                    }
                } else {
                    log::warn!("Rule with `delete` action for label `{label}` skipped.");
                }
            }
        }
    }
}
