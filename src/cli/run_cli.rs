use clap::Parser;
use cull_gmail::{Config, EolAction, Result};

#[derive(Debug, Parser)]
pub struct RunCli {}

impl RunCli {
    pub async fn run(&self, config: Config) -> Result<()> {
        let rules = config.get_rules_by_label();

        for label in config.labels() {
            let Some(rule) = rules.get(&label) else {
                log::warn!("no rule found for label `{label}`");
                continue;
            };

            log::info!("Executing rule `#{}` for label `{label}`", rule.describe());

            let Some(action) = rule.action() else {
                log::warn!("no valid action specified for rule #{}", rule.id());
                continue;
            };

            match action {
                EolAction::Trash => log::info!("trashing older messages"),
                EolAction::Delete => log::info!("deleting older messages"),
            }
        }

        Ok(())
    }
}
