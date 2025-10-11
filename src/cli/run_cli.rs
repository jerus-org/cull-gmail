use clap::Parser;
use cull_gmail::{Config, EolAction, Processor, Result};

#[derive(Debug, Parser)]
pub struct RunCli {
    /// Skip any rules that apply the action `trash`
    #[clap(short = 't', long)]
    skip_trash: bool,
}

impl RunCli {
    pub async fn run(&self, config: Config) -> Result<()> {
        let rules = config.get_rules_by_label();

        for label in config.labels() {
            let Some(rule) = rules.get(&label) else {
                log::warn!("no rule found for label `{label}`");
                continue;
            };

            log::info!("Executing rule `#{}` for label `{label}`", rule.describe());

            let processor = Processor::new(config.credential_file(), rule);

            let Some(action) = processor.action() else {
                log::warn!("no valid action specified for rule #{}", rule.id());
                continue;
            };

            match action {
                EolAction::Trash => {
                    if !self.skip_trash {
                        log::info!("trashing older messages");
                        match processor.trash_messages(&label).await {
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
                    log::info!("deleting older messages");
                    match processor.delete_messages(&label).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::warn!("action failed for label {label} with error {e}");
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
