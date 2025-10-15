use clap::{Parser, Subcommand};

mod labels_cli;
mod messages_cli;
mod rules_cli;

use cull_gmail::{Config, GmailClient, Result};
use std::error::Error as stdError;

use labels_cli::LabelsCli;
use messages_cli::MessagesCli;
use rules_cli::RulesCli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    sub_command: SubCmds,
}

#[derive(Subcommand, Debug)]
enum SubCmds {
    /// List messages
    #[clap(name = "messages", display_order = 3, next_help_heading = "Labels")]
    Message(MessagesCli),
    /// List labels
    #[clap(name = "labels", display_order = 2, next_help_heading = "Rules")]
    Labels(LabelsCli),
    /// Configure and run rules
    #[clap(name = "rules", display_order = 2)]
    Rules(RulesCli),
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let mut logging = get_logging(args.logging.log_level_filter());
    logging.init();
    log::info!("Logging started.");

    std::process::exit(match run(args).await {
        Ok(_) => 0,
        Err(e) => {
            if let Some(src) = e.source() {
                log::error!("{e}: {src}");
                eprintln!("{e}: {src}");
            } else {
                log::error!("{e}");
                eprintln!("{e}");
            }
            101
        }
    });
}

async fn run(args: Cli) -> Result<()> {
    let config = get_config()?;
    log::trace!("Configuration loaded: {config:#?}");

    let mut client = GmailClient::new(config.credential_file()).await?;

    match args.sub_command {
        SubCmds::Message(messages_cli) => messages_cli.run(&mut client).await,
        SubCmds::Labels(labels_cli) => labels_cli.run(client).await,
        SubCmds::Rules(rules_cli) => rules_cli.run(&mut client, config).await,
    }
}

fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    let level = if level > log::LevelFilter::Info {
        level
    } else {
        log::LevelFilter::Info
    };

    let mut builder = env_logger::Builder::new();

    builder.filter(Some("cull_gmail"), level);
    // TODO: Provide an option to set wider filter allowing all crates to report

    builder.format_timestamp_secs().format_module_path(false);

    builder
}

fn get_config() -> Result<Config> {
    match Config::load() {
        Ok(c) => Ok(c),
        Err(_) => {
            log::warn!("Configuration not found, creating default config.");
            let config = Config::new();
            config.save()?;
            Ok(config)
        }
    }
}
