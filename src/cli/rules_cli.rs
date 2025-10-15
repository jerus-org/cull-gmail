use clap::{Parser, Subcommand};

mod config_cli;
mod run_cli;

use cull_gmail::{Config, GmailClient, Result};

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
    pub async fn run(&self, client: &mut GmailClient, config: Config) -> Result<()> {
        match &self.sub_command {
            SubCmds::Config(config_cli) => config_cli.run(config),
            SubCmds::Run(run_cli) => run_cli.run(client, config).await,
        }
    }
}
