use clap::{Parser, Subcommand};

mod labels_cli;
mod messages_cli;
mod rules_cli;

use config::Config;
use cull_gmail::{ClientConfig, EolAction, GmailClient, Result, RuleProcessor, Rules};
use std::{env, error::Error as stdError};

use labels_cli::LabelsCli;
use messages_cli::MessagesCli;
use rules_cli::RulesCli;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,
    #[command(subcommand)]
    sub_command: Option<SubCmds>,
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
    let (config, client_config) = get_config()?;

    let mut client = GmailClient::new_with_config(client_config).await?;

    let Some(sub_command) = args.sub_command else {
        let rules = rules_cli::get_rules()?;
        let execute = config.get_bool("execute").unwrap_or(false);
        return run_rules(&mut client, rules, execute).await;
    };

    match sub_command {
        SubCmds::Message(messages_cli) => messages_cli.run(&mut client).await,
        SubCmds::Labels(labels_cli) => labels_cli.run(client).await,
        SubCmds::Rules(rules_cli) => rules_cli.run(&mut client).await,
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

fn get_config() -> Result<(Config, ClientConfig)> {
    let home_dir = env::home_dir().unwrap();
    let path = home_dir.join(".cull-gmail/cull-gmail.toml");
    log::info!("Loading config from {}", path.display());

    let configurations = config::Config::builder()
        .set_default("credentials", "credential.json")?
        .set_default("config_root", "h:.cull-gmail")?
        .set_default("rules", "rules.toml")?
        .set_default("execute", true)?
        .add_source(config::File::with_name(
            path.to_path_buf().to_str().unwrap(),
        ))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    Ok((
        configurations.clone(),
        ClientConfig::new_from_configuration(configurations)?,
    ))
}

async fn run_rules(client: &mut GmailClient, rules: Rules, execute: bool) -> Result<()> {
    let rules_by_labels = rules.get_rules_by_label();

    for label in rules.labels() {
        let Some(rule) = rules_by_labels.get(&label) else {
            log::warn!("no rule found for label `{label}`");
            continue;
        };

        log::info!("Executing rule `#{}` for label `{label}`", rule.describe());
        client.set_rule(rule.clone());
        client.set_execute(execute);
        if let Err(e) = client.find_rule_and_messages_for_label(&label).await {
            log::warn!("Nothing to process for label `{label}` as {e}");
            continue;
        }
        let Some(action) = client.action() else {
            log::warn!("no valid action specified for rule #{}", rule.id());
            continue;
        };

        if execute {
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
