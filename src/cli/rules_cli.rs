use clap::{Parser, Subcommand};

mod config_cli;
mod run_cli;

use cull_gmail::{GmailClient, Result, Rules};

use config_cli::ConfigCli;
use run_cli::RunCli;

#[derive(Subcommand, Debug)]
enum SubCmds {
    /// Configure end-of-life rules
    #[clap(name = "config")]
    Config(ConfigCli),
    /// Run end-of-life rules
    #[clap(name = "run")]
    Run(RunCli),
}

#[derive(Debug, Parser)]
pub struct RulesCli {
    #[command(subcommand)]
    sub_command: SubCmds,
}

impl RulesCli {
    pub async fn run(&self, client: &mut GmailClient) -> Result<()> {
        let rules = get_rules()?;

        match &self.sub_command {
            SubCmds::Config(config_cli) => config_cli.run(rules),
            SubCmds::Run(run_cli) => run_cli.run(client, rules).await,
        }
    }
}

pub fn get_rules() -> Result<Rules> {
    match Rules::load() {
        Ok(c) => Ok(c),
        Err(_) => {
            log::warn!("Configuration not found, creating default config.");
            let rules = Rules::new();
            rules.save()?;
            Ok(rules)
        }
    }
}
