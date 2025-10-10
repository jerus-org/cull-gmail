use clap::{Parser, Subcommand};

mod config_cli;
mod delete_cli;
mod label_cli;
mod message_cli;
mod run_cli;
mod trash_cli;

use cull_gmail::{Config, Result};

use config_cli::ConfigCli;
use delete_cli::DeleteCli;
use label_cli::LabelCli;
use message_cli::MessageCli;
use run_cli::RunCli;
use trash_cli::TrashCli;

use std::error::Error as stdError;

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
    /// Configure rules and labels
    #[clap(name = "config")]
    Config(ConfigCli),
    /// List messages
    #[clap(name = "message")]
    Message(MessageCli),
    /// List labels
    #[clap(name = "label")]
    Labels(LabelCli),
    /// Move messages to trash
    #[clap(name = "trash")]
    Trash(TrashCli),
    /// Delete messages
    #[clap(name = "delete")]
    Delete(DeleteCli),
    /// Run the rules from the rules configuration
    #[clap(name = "run")]
    Run(RunCli),
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
    match args.sub_command {
        SubCmds::Config(config_cli) => config_cli.run(config),
        SubCmds::Message(list_cli) => list_cli.run(config.credential_file()).await,
        SubCmds::Labels(label_cli) => label_cli.run(config.credential_file()).await,
        SubCmds::Trash(trash_cli) => trash_cli.run(config.credential_file()).await,
        SubCmds::Delete(delete_cli) => delete_cli.run(config.credential_file()).await,
        SubCmds::Run(run_cli) => run_cli.run(config).await,
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
