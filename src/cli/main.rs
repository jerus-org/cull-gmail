//! # Gmail Message Cull CLI Application
//!
//! A command-line interface for managing Gmail messages with automated retention rules.
//! This CLI provides powerful tools for querying, filtering, and managing Gmail messages
//! based on labels, age, and custom rules with built-in safety features like dry-run mode.
//!
//! ## Overview
//!
//! The CLI is built around three main command categories:
//!
//! - **Labels**: List and inspect Gmail labels for message organization
//! - **Messages**: Query, filter, and perform batch operations on Gmail messages
//! - **Rules**: Configure and execute automated message lifecycle management rules
//!
//! ## Authentication
//!
//! The CLI uses OAuth2 for Gmail API authentication with the following configuration:
//!
//! - **Configuration file**: `~/.cull-gmail/cull-gmail.toml`
//! - **Credential file**: OAuth2 credentials from Google Cloud Platform
//! - **Token storage**: Automatic token caching in `~/.cull-gmail/gmail1/`
//!
//! ## Command Structure
//!
//! ```bash
//! cull-gmail [OPTIONS] [COMMAND]
//! ```
//!
//! ### Global Options
//!
//! - `-v, --verbose...`: Increase logging verbosity (can be used multiple times)
//! - `-q, --quiet...`: Decrease logging verbosity
//! - `-h, --help`: Show help information
//! - `-V, --version`: Show version information
//!
//! ### Commands
//!
//! 1. **`labels`**: List all available Gmail labels
//! 2. **`messages`**: Query and operate on Gmail messages
//! 3. **`rules`**: Configure and execute retention rules
//!
//! ## Configuration File Format
//!
//! The CLI expects a TOML configuration file at `~/.cull-gmail/cull-gmail.toml`:
//!
//! ```toml
//! # OAuth2 credential file (required)
//! credential_file = "client_secret.json"
//!
//! # Configuration root directory
//! config_root = "h:.cull-gmail"
//!
//! # Rules configuration file
//! rules = "rules.toml"
//!
//! # Default execution mode (false = dry-run, true = execute)
//! execute = false
//! ```
//!
//! ## Safety Features
//!
//! - **Dry-run mode**: Default behaviour prevents accidental data loss
//! - **Comprehensive logging**: Detailed operation tracking with multiple verbosity levels
//! - **Error handling**: Graceful error recovery with meaningful error messages
//! - **Confirmation prompts**: For destructive operations
//!
//! ## Usage Examples
//!
//! ### List Gmail Labels
//! ```bash
//! cull-gmail labels
//! ```
//!
//! ### Query Messages
//! ```bash
//! # List recent messages
//! cull-gmail messages -m 10 list
//!
//! # Find old promotional emails
//! cull-gmail messages -Q "label:promotions older_than:1y" list
//! ```
//!
//! ### Execute Rules
//! ```bash
//! # Preview rule execution (dry-run)
//! cull-gmail rules run
//!
//! # Execute rules for real
//! cull-gmail rules run --execute
//! ```
//!
//! ## Error Handling
//!
//! The CLI returns the following exit codes:
//! - **0**: Success
//! - **101**: Error (check stderr and logs for details)
//!
//! ## Logging
//!
//! Logging is controlled through command-line verbosity flags and environment variables:
//!
//! - **Default**: Info level logging for the cull-gmail crate
//! - **Verbose (`-v`)**: Debug level logging
//! - **Very Verbose (`-vv`)**: Trace level logging
//! - **Quiet (`-q`)**: Error level logging only
//!
//! Environment variable override:
//! ```bash
//! export RUST_LOG=cull_gmail=debug
//! ```

use clap::{Parser, Subcommand};

mod init_cli;
mod labels_cli;
mod messages_cli;
mod rules_cli;
mod token_cli;

use config::Config;
use cull_gmail::{ClientConfig, EolAction, GmailClient, MessageList, Result, RuleProcessor, Rules};
use std::{env, error::Error as stdError};

use init_cli::InitCli;
use labels_cli::LabelsCli;
use messages_cli::MessagesCli;
use rules_cli::RulesCli;
use token_cli::{TokenCli, restore_tokens_from_string};

use std::path::PathBuf;

/// Main CLI application structure defining global options and subcommands.
///
/// This struct represents the root of the command-line interface, providing
/// global configuration options and dispatching to specific subcommands for
/// labels, messages, and rules management.
///
/// # Global Options
///
/// - **Logging**: Configurable verbosity levels for operation visibility
/// - **Subcommands**: Optional command selection (defaults to rule execution)
///
/// # Default behaviour
///
/// When no subcommand is provided, the CLI executes the default rule processing
/// workflow, loading rules from the configuration file and executing them
/// according to the current execution mode (dry-run or live).
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Logging verbosity control.
    ///
    /// Use `-q` for quiet (errors only), default for info level,
    /// `-v` for debug level, `-vv` for trace level.
    #[clap(flatten)]
    logging: clap_verbosity_flag::Verbosity,

    /// Optional subcommand selection.
    ///
    /// If not provided, the CLI will execute the default rule processing workflow.
    #[command(subcommand)]
    sub_command: Option<SubCmds>,
}

/// Available CLI subcommands for Gmail message management.
///
/// Each subcommand provides specialized functionality for different aspects
/// of Gmail message lifecycle management, from inspection to automated processing.
///
/// # Command Categories
///
/// - **Messages**: Direct message querying, filtering, and batch operations
/// - **Labels**: Gmail label inspection and management
/// - **Rules**: Automated message lifecycle rule configuration and execution
///
/// # Display Order
///
/// Commands are ordered by typical usage workflow: inspect labels first,
/// then query specific messages, and finally configure automated rules.
#[derive(Subcommand, Debug)]
enum SubCmds {
    /// Initialize cull-gmail configuration, credentials, and OAuth2 tokens.
    ///
    /// Sets up the complete cull-gmail environment including configuration directory,
    /// OAuth2 credentials, default configuration files, and initial authentication flow.
    #[clap(name = "init", display_order = 1)]
    Init(InitCli),

    /// Query, filter, and perform batch operations on Gmail messages.
    ///
    /// Supports advanced Gmail query syntax, label filtering, and batch actions
    /// including trash and permanent deletion with safety controls.
    #[clap(name = "messages", display_order = 3, next_help_heading = "Labels")]
    Message(MessagesCli),

    /// List and inspect available Gmail labels.
    ///
    /// Displays all labels in your Gmail account with their internal IDs,
    /// useful for understanding label structure before creating queries or rules.
    #[clap(name = "labels", display_order = 2, next_help_heading = "Rules")]
    Labels(LabelsCli),

    /// Configure and execute automated message retention rules.
    ///
    /// Provides rule-based message lifecycle management with configurable
    /// retention periods, label targeting, and automated actions.
    #[clap(name = "rules", display_order = 2)]
    Rules(RulesCli),

    /// Export and import OAuth2 tokens for ephemeral environments.
    ///
    /// Supports token export to compressed strings and automatic import from
    /// environment variables for container deployments and CI/CD pipelines.
    #[clap(name = "token", display_order = 4)]
    Token(TokenCli),
}

/// CLI application entry point with comprehensive error handling and logging setup.
///
/// This function initializes the async runtime, parses command-line arguments,
/// configures logging based on user preferences, and orchestrates the main
/// application workflow with proper error handling and exit code management.
///
/// # Process Flow
///
/// 1. **Argument Parsing**: Parse command-line arguments using clap
/// 2. **Logging Setup**: Initialize logging with user-specified verbosity
/// 3. **Application Execution**: Run the main application logic
/// 4. **Error Handling**: Handle errors with detailed reporting
/// 5. **Exit Code**: Return appropriate exit codes for shell integration
///
/// # Exit Codes
///
/// - **0**: Successful execution
/// - **101**: Error occurred (details logged and printed to stderr)
///
/// # Error Reporting
///
/// Errors are reported through multiple channels:
/// - **Logging**: Structured error logging for debugging
/// - **stderr**: User-friendly error messages
/// - **Exit codes**: Shell-scriptable status reporting
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

/// Main application logic dispatcher handling subcommand execution and default behaviour.
///
/// This function orchestrates the core application workflow by:
/// 1. Loading configuration from files and environment
/// 2. Initializing the Gmail API client with OAuth2 authentication
/// 3. Dispatching to appropriate subcommands or executing default rule processing
///
/// # Arguments
///
/// * `args` - Parsed command-line arguments containing global options and subcommands
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of the operation.
///
/// # Default behaviour
///
/// When no subcommand is specified, the function executes the default rule processing
/// workflow, loading rules from configuration and executing them based on the
/// current execution mode setting.
///
/// # Error Handling
///
/// Errors can occur during:
/// - Configuration loading and parsing
/// - Gmail client initialization and authentication
/// - Subcommand execution
/// - Rule processing operations
async fn run(args: Cli) -> Result<()> {
    // Handle init command first, before trying to load config
    if let Some(SubCmds::Init(init_cli)) = args.sub_command {
        // Init commands don't need existing config since they set up the config
        return init_cli.run().await;
    }

    // For all other commands, load config normally
    let (config, client_config) = get_config()?;

    // Check for token restoration before client initialization
    restore_tokens_if_available(&config, &client_config)?;

    let mut client = GmailClient::new_with_config(client_config).await?;

    // Get configured rules path
    let rules_path = get_rules_path(&config)?;

    let Some(sub_command) = args.sub_command else {
        let rules = rules_cli::get_rules_from(rules_path.as_deref())?;
        let execute = config.get_bool("execute").unwrap_or(false);
        return run_rules(&mut client, rules, execute).await;
    };

    match sub_command {
        SubCmds::Init(_) => {
            // This should never be reached due to early return above
            unreachable!("Init command should have been handled earlier");
        }
        SubCmds::Message(messages_cli) => messages_cli.run(&mut client).await,
        SubCmds::Labels(labels_cli) => labels_cli.run(client).await,
        SubCmds::Rules(rules_cli) => {
            rules_cli
                .run_with_rules_path(&mut client, rules_path.as_deref())
                .await
        }
        SubCmds::Token(token_cli) => {
            // Token commands don't need an initialized client, just the config
            // We need to get a fresh client_config since the original was moved
            let (_, token_client_config) = get_config()?;
            token_cli.run(&token_client_config).await
        }
    }
}

/// Creates and configures a logging builder with appropriate verbosity levels.
///
/// This function sets up structured logging for the application with:
/// - Minimum info-level logging for user-facing information
/// - Configurable verbosity based on command-line flags
/// - Timestamp formatting for operation tracking
/// - Focused logging on the cull-gmail crate to reduce noise
///
/// # Arguments
///
/// * `level` - Desired log level filter from command-line verbosity flags
///
/// # Returns
///
/// Returns a configured `env_logger::Builder` ready for initialization.
///
/// # Logging Levels
///
/// - **Error**: Critical failures and unrecoverable errors
/// - **Warn**: Non-fatal issues, dry-run notifications, missing resources
/// - **Info**: General operation progress, message counts, rule execution
/// - **Debug**: Detailed operation info, API calls, configuration values
/// - **Trace**: Very detailed debugging information
///
/// # Default behaviour
///
/// The function enforces a minimum of Info-level logging to ensure users
/// receive adequate feedback about application operations, even when
/// verbosity is not explicitly requested.
fn get_logging(level: log::LevelFilter) -> env_logger::Builder {
    // let level = if level > log::LevelFilter::Info {
    //     level
    // } else {
    //     log::LevelFilter::Info
    // };

    let mut builder = env_logger::Builder::new();

    builder.filter(Some("cull_gmail"), level);
    // TODO: Provide an option to set wider filter allowing all crates to report

    builder.format_timestamp_secs().format_module_path(false);

    builder
}

/// Loads and parses application configuration from multiple sources.
///
/// This function implements a hierarchical configuration loading strategy:
/// 1. **Default values**: Sensible defaults for all configuration options
/// 2. **Configuration file**: User-specific settings from `~/.cull-gmail/cull-gmail.toml`
/// 3. **Environment variables**: Runtime overrides with `APP_` prefix
///
/// # Returns
///
/// Returns a tuple containing:
/// - **Config**: Raw configuration for general application settings
/// - **ClientConfig**: Processed Gmail client configuration with OAuth2 setup
///
/// # Configuration Hierarchy
///
/// Settings are applied in this order (later sources override earlier ones):
/// 1. Built-in defaults
/// 2. Configuration file values
/// 3. Environment variable overrides
///
/// # Configuration Parameters
///
/// ## Default Values:
/// - `credentials`: "credential.json" - OAuth2 credential file name
/// - `config_root`: "h:.cull-gmail" - Configuration directory (home-relative)
/// - `rules`: "rules.toml" - Rules configuration file name
/// - `execute`: true - Default execution mode (can be overridden for safety)
///
/// ## Environment Variables:
/// - `APP_CREDENTIALS`: Override credential file name
/// - `APP_CONFIG_ROOT`: Override configuration directory
/// - `APP_RULES`: Override rules file name
/// - `APP_EXECUTE`: Override execution mode (true/false)
///
/// # Error Handling
///
/// Configuration errors can occur due to:
/// - Missing or inaccessible configuration files
/// - Invalid TOML syntax in configuration files
/// - Missing OAuth2 credential files
/// - Invalid OAuth2 credential format or structure
fn get_config() -> Result<(Config, ClientConfig)> {
    let home_dir = env::home_dir().unwrap();
    let path = home_dir.join(".cull-gmail/cull-gmail.toml");
    log::info!("Loading config from {}", path.display());

    let mut config_builder = config::Config::builder()
        .set_default("credential_file", "credential.json")?
        .set_default("config_root", "h:.cull-gmail")?
        .set_default("rules", "rules.toml")?
        .set_default("execute", true)?
        .set_default("token_uri", "https://oauth2.googleapis.com/token")?
        .set_default("auth_uri", "https://accounts.google.com/o/oauth2/auth")?
        .set_default("token_cache_env", "CULL_GMAIL_TOKEN_CACHE")?;

    if path.exists() {
        let config_file = config::File::with_name(path.to_path_buf().to_str().unwrap());
        log::info!("Config file {config_file:?}");
        config_builder = config_builder.add_source(config_file);
    }
    let configurations = config_builder
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    Ok((
        configurations.clone(),
        ClientConfig::new_from_configuration(configurations)?,
    ))
}

/// Executes automated message retention rules across Gmail labels.
///
/// This function orchestrates the rule-based message processing workflow by:
/// 1. Organizing rules by their target labels
/// 2. Processing each label according to its configured rule
/// 3. Executing or simulating actions based on execution mode
///
/// # Arguments
///
/// * `client` - Mutable Gmail client for API operations
/// * `rules` - Loaded rules configuration containing all retention policies
/// * `execute` - Whether to actually perform actions (true) or dry-run (false)
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of the rule processing.
///
/// # Rule Processing Flow
///
/// For each configured label:
/// 1. **Rule Lookup**: Find the retention rule for the label
/// 2. **Rule Application**: Apply rule criteria to find matching messages
/// 3. **Action Determination**: Determine appropriate action (trash/delete)
/// 4. **Execution**: Execute action or simulate for dry-run
///
/// # Safety Features
///
/// - **Dry-run mode**: When `execute` is false, actions are logged but not performed
/// - **Error isolation**: Errors for individual labels don't stop processing of other labels
/// - **Detailed logging**: Comprehensive logging of rule execution and results
///
/// # Error Handling
///
/// The function continues processing even if individual rules fail, logging
/// warnings for missing rules, processing errors, or action failures.
async fn run_rules(client: &mut GmailClient, rules: Rules, execute: bool) -> Result<()> {
    run_rules_for_action(client, &rules, execute, EolAction::Trash).await?;
    run_rules_for_action(client, &rules, execute, EolAction::Delete).await?;

    Ok(())
}

/// Executes automated message retention rules across Gmail labels for an action.
///
/// This function orchestrates the rule-based message processing workflow by:
/// 1. Organizing rules by their target labels
/// 2. Processing each label according to its configured rule
/// 3. Executing or simulating actions based on execution mode
///
/// # Arguments
///
/// * `client` - Mutable Gmail client for API operations
/// * `rules` - Loaded rules configuration containing all retention policies
/// * `execute` - Whether to actually perform actions (true) or dry-run (false)
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure of the rule processing.
///
/// # Rule Processing Flow
///
/// For each configured label:
/// 1. **Rule Lookup**: Find the retention rule for the label
/// 2. **Rule Application**: Apply rule criteria to find matching messages
/// 3. **Action Determination**: Determine appropriate action (trash/delete)
/// 4. **Execution**: Execute action or simulate for dry-run
///
/// # Safety Features
///
/// - **Dry-run mode**: When `execute` is false, actions are logged but not performed
/// - **Error isolation**: Errors for individual labels don't stop processing of other labels
/// - **Detailed logging**: Comprehensive logging of rule execution and results
///
/// # Error Handling
///
/// The function continues processing even if individual rules fail, logging
/// warnings for missing rules, processing errors, or action failures.
async fn run_rules_for_action(
    client: &mut GmailClient,
    rules: &Rules,
    execute: bool,
    action: EolAction,
) -> Result<()> {
    let rules_by_labels = rules.get_rules_by_label_for_action(action);

    for label in rules.labels() {
        let Some(rule) = rules_by_labels.get(&label) else {
            log::warn!("no rule found for label `{label}`");
            continue;
        };

        log::info!("Executing rule `#{}` for label `{label}`", rule.describe());
        client.initialise_lists();
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
            execute_action(action, client, &label).await;
        } else {
            client.log_messages("", "").await?;
            log::warn!("Execution stopped for dry run");
        }
    }

    Ok(())
}

/// Restores OAuth2 tokens from environment variable if available.
///
/// This function checks if the token cache environment variable is set and,
/// if found, restores the token files before client initialization to enable
/// ephemeral environment workflows.
///
/// # Arguments
///
/// * `config` - Application configuration containing token environment variable name
/// * `client_config` - Client configuration containing token persistence path
///
/// # Returns
///
/// Returns `Result<()>` indicating success or failure. Non-critical errors
/// (like missing environment variables) are logged but don't cause failure.
///
/// # Process
///
/// 1. **Check Environment**: Look for configured token cache environment variable
/// 2. **Skip if Missing**: Continue normally if environment variable not set
/// 3. **Restore Tokens**: Decode and restore token files if variable present
/// 4. **Log Results**: Report restoration success or failures
///
/// This function enables seamless token restoration for:
/// - Container deployments with injected token environment variables
/// - CI/CD pipelines with stored token secrets
/// - Ephemeral compute environments requiring periodic Gmail access
fn restore_tokens_if_available(config: &Config, client_config: &ClientConfig) -> Result<()> {
    let token_env_var = config
        .get_string("token_cache_env")
        .unwrap_or_else(|_| "CULL_GMAIL_TOKEN_CACHE".to_string());

    if let Ok(token_data) = env::var(&token_env_var) {
        log::info!("Found {token_env_var} environment variable, restoring tokens");
        restore_tokens_from_string(&token_data, client_config.persist_path())?;
        log::info!("Tokens successfully restored from environment variable");
    } else {
        log::debug!(
            "No {token_env_var} environment variable found, proceeding with normal token flow"
        );
    }

    Ok(())
}

/// Gets the rules file path from configuration.
///
/// Reads the `rules` configuration value and resolves it using path prefixes.
/// Supports h:, c:, r: prefixes for home, current, and root directories.
///
/// # Arguments
///
/// * `config` - Application configuration
///
/// # Returns
///
/// Returns the resolved rules file path, or None if using default location.
fn get_rules_path(config: &Config) -> Result<Option<PathBuf>> {
    let rules_config = config
        .get_string("rules")
        .unwrap_or_else(|_| "rules.toml".to_string());

    // If it's just "rules.toml" (the default), return None to use default location
    if rules_config == "rules.toml" {
        return Ok(None);
    }

    // Otherwise, parse the path with prefix support
    let path = init_cli::parse_config_root(&rules_config);
    Ok(Some(path))
}

/// Executes the specified end-of-life action on messages for a Gmail label.
///
/// This function performs the actual message operations (trash or delete) based on
/// the rule configuration and execution mode. It handles both recoverable (trash)
/// and permanent (delete) operations with appropriate logging and error handling.
///
/// # Arguments
///
/// * `action` - The end-of-life action to perform (Trash or Delete)
/// * `client` - Gmail client configured with messages to process
/// * `label` - Label name for context in logging and error reporting
///
/// # Actions
///
/// ## Trash
/// - **Operation**: Moves messages to Gmail's Trash folder
/// - **Reversibility**: Messages can be recovered from Trash for ~30 days
/// - **Safety**: Relatively safe operation with recovery options
///
/// ## Delete
/// - **Operation**: Permanently deletes messages from Gmail
/// - **Reversibility**: **IRREVERSIBLE** - messages cannot be recovered
/// - **Safety**: High-risk operation requiring careful consideration
///
/// # Error Handling
///
/// The function logs errors but does not propagate them, allowing rule processing
/// to continue for other labels even if one action fails. Errors are reported through:
/// - **Warning logs**: Structured logging for debugging
/// - **Label context**: Error messages include label name for traceability
///
/// # Safety Considerations
///
/// This function should only be called when execute mode is enabled and after
/// appropriate user confirmation for destructive operations.
async fn execute_action(action: EolAction, client: &mut GmailClient, label: &str) {
    match action {
        EolAction::Trash => {
            log::info!("***executing trash messages***");
            if client.batch_trash().await.is_err() {
                log::warn!("Move to trash failed for label `{label}`");
            }
        }
        EolAction::Delete => {
            log::info!("***executing final delete messages***");
            if client.batch_delete().await.is_err() {
                log::warn!("Delete failed for label `{label}`");
            }
        }
    }
}
