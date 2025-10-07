use clap::{Parser, Subcommand};

mod config_cli;
mod label_cli;
mod message_cli;
mod trash_cli;

use config_cli::ConfigCli;
use cull_gmail::Error;
use label_cli::LabelCli;
use message_cli::MessageCli;
use std::error::Error as stdError;

use trash_cli::TrashCli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List messages
    #[clap(name = "message")]
    Message(MessageCli),
    /// List labels
    #[clap(name = "label")]
    Labels(LabelCli),
    /// Move messages to trash
    #[clap(name = "trash")]
    Trash(TrashCli),
    /// Configure end-of-life rules
    #[clap(name = "config")]
    Config(ConfigCli),
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

async fn run(args: Cli) -> Result<(), Error> {
    if let Some(cmds) = args.command {
        match cmds {
            Commands::Message(list_cli) => list_cli.run("credential.json").await?,
            Commands::Labels(label_cli) => label_cli.run("credential.json").await?,
            Commands::Trash(trash_cli) => trash_cli.run("credential.json").await?,
            Commands::Config(config_cli) => config_cli.run(),
        }
    }
    Ok(())
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
