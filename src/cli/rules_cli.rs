//! # Gmail Rules CLI Module
//!
//! This module provides command-line interface functionality for configuring and executing
//! automated Gmail message retention rules. It enables users to create sophisticated
//! message lifecycle policies with configurable retention periods, label targeting,
//! and automated actions (trash/delete).
//!
//! ## Overview
//!
//! The rules command provides two main functionalities:
//! - **Configuration**: Create, modify, and manage retention rules
//! - **Execution**: Run configured rules to process Gmail messages automatically
//!
//! ## Rule-Based Message Management
//!
//! Rules enable automated message lifecycle management by:
//! - **Time-based filtering**: Target messages based on age criteria
//! - **Label-based targeting**: Apply rules to specific Gmail labels
//! - **Automated actions**: Perform trash or delete operations
//! - **Safety controls**: Built-in dry-run and logging capabilities
//!
//! ## Command Structure
//!
//! ```bash
//! cull-gmail rules <SUBCOMMAND>
//! ```
//!
//! ### Available Subcommands
//!
//! - **`config`**: Configure retention rules, labels, and actions
//! - **`run`**: Execute configured rules with optional safety controls
//!
//! ## Rule Configuration
//!
//! Rules are stored in TOML format with the following structure:
//!
//! ```toml
//! [rules."1"]
//! id = 1
//! retention = { age = "y:2", generate_label = true }
//! labels = ["old-emails"]
//! action = "Trash"
//!
//! [rules."2"]
//! id = 2
//! retention = { age = "m:6", generate_label = true }
//! labels = ["promotions", "newsletters"]
//! action = "Delete"
//! ```
//!
//! ## Retention Periods
//!
//! Supported time formats:
//! - **Years**: `y:1`, `y:2`, etc.
//! - **Months**: `m:6`, `m:12`, etc.
//! - **Days**: `d:30`, `d:90`, etc.
//!
//! ## Actions
//!
//! - **Trash**: Move messages to recoverable Trash folder (~30 day recovery)
//! - **Delete**: Permanently remove messages (irreversible)
//!
//! ## Safety Features
//!
//! - **Dry-run mode**: Default execution mode prevents accidental data loss
//! - **Rule validation**: Configuration validation before execution
//! - **Comprehensive logging**: Detailed operation tracking
//! - **Error isolation**: Individual rule failures don't stop processing
//!
//! ## Usage Examples
//!
//! ### Configure Rules
//! ```bash
//! # Add a new rule
//! cull-gmail rules config rules add
//!
//! # Configure rule labels
//! cull-gmail rules config label add 1 "old-emails"
//!
//! # Set rule action
//! cull-gmail rules config action 1 trash
//! ```
//!
//! ### Execute Rules
//! ```bash
//! # Dry-run (safe preview)
//! cull-gmail rules run
//!
//! # Execute for real
//! cull-gmail rules run --execute
//!
//! # Execute only specific action types
//! cull-gmail rules run --execute --skip-delete
//! ```
//!
//! ## Integration
//!
//! This module integrates with:
//! - **Rules engine**: Core rule processing and validation
//! - **GmailClient**: Message querying and batch operations
//! - **Configuration system**: TOML-based rule persistence
//! - **Logging system**: Comprehensive operation tracking

use clap::{Parser, Subcommand};
use std::path::Path;

mod config_cli;
mod run_cli;

use cull_gmail::{GmailClient, Result, Rules};

use config_cli::ConfigCli;
use run_cli::RunCli;

/// Available subcommands for rules management and execution.
///
/// This enum defines the two main operational modes for the rules CLI:
/// configuration management and rule execution. Each mode provides
/// specialized functionality for different aspects of rule lifecycle management.
///
/// # Command Categories
///
/// - **Config**: Rule definition, modification, and management operations
/// - **Run**: Rule execution with various safety and control options
///
/// # Workflow Integration
///
/// Typical usage follows this pattern:
/// 1. Use `config` to set up rules, labels, and actions
/// 2. Use `run` to execute rules with dry-run testing
/// 3. Use `run --execute` for live rule execution
#[derive(Subcommand, Debug)]
enum SubCmds {
    /// Configure Gmail message retention rules, labels, and actions.
    ///
    /// Provides comprehensive rule management functionality including:
    /// - **Rule creation**: Define new retention policies
    /// - **Label management**: Configure target labels for rules
    /// - **Action setting**: Specify trash or delete actions
    /// - **Rule modification**: Update existing rule parameters
    ///
    /// The config subcommand enables fine-grained control over rule behaviour
    /// and provides validation to ensure rules are properly configured
    /// before execution.
    #[clap(name = "config")]
    Config(ConfigCli),

    /// Execute configured retention rules with optional safety controls.
    ///
    /// Provides rule execution functionality with comprehensive safety features:
    /// - **Dry-run mode**: Preview rule effects without making changes
    /// - **Selective execution**: Skip specific action types (trash/delete)
    /// - **Error handling**: Continue processing despite individual failures
    /// - **Progress tracking**: Detailed logging of rule execution
    ///
    /// The run subcommand is the primary interface for automated message
    /// lifecycle management based on configured retention policies.
    #[clap(name = "run")]
    Run(RunCli),
}

/// Command-line interface for Gmail message retention rule management.
///
/// This structure represents the rules subcommand, providing comprehensive
/// functionality for both configuring and executing automated Gmail message
/// retention policies. It serves as the main entry point for rule-based
/// message lifecycle management.
///
/// # Core Functionality
///
/// - **Rule Configuration**: Create, modify, and manage retention rules
/// - **Label Management**: Associate rules with specific Gmail labels
/// - **Action Control**: Configure trash or delete actions for rules
/// - **Rule Execution**: Run configured rules with safety controls
///
/// # Architecture
///
/// The RulesCli delegates to specialized subcommands:
/// - **ConfigCli**: Handles all rule configuration operations
/// - **RunCli**: Manages rule execution and safety controls
///
/// # Configuration Flow
///
/// 1. **Rule Creation**: Define retention periods and basic parameters
/// 2. **Label Assignment**: Associate rules with target Gmail labels
/// 3. **Action Configuration**: Set appropriate actions (trash/delete)
/// 4. **Validation**: Ensure rules are properly configured
/// 5. **Execution**: Run rules with appropriate safety controls
///
/// # Safety Integration
///
/// The RulesCli incorporates multiple safety layers:
/// - **Configuration validation**: Rules are validated before execution
/// - **Dry-run capabilities**: Preview rule effects before applying changes
/// - **Error isolation**: Individual rule failures don't stop processing
/// - **Comprehensive logging**: Detailed tracking of all operations
///
/// # Usage Context
///
/// This CLI is designed for:
/// - **System administrators**: Managing organizational Gmail retention policies
/// - **Power users**: Implementing personal email organization strategies
/// - **Automation**: Scheduled execution of maintenance tasks
/// - **Compliance**: Meeting data retention requirements
#[derive(Debug, Parser)]
pub struct RulesCli {
    /// Subcommand selection for rules operations.
    ///
    /// Determines whether to perform configuration management or rule execution.
    /// Each subcommand provides specialized functionality for its domain.
    #[command(subcommand)]
    sub_command: SubCmds,
}

impl RulesCli {
    /// Executes the rules command based on the selected subcommand.
    ///
    /// This method coordinates the rules workflow by first loading the current
    /// rule configuration, then dispatching to the appropriate subcommand handler
    /// based on user selection (config or run).
    ///
    /// # Arguments
    ///
    /// * `client` - Mutable Gmail client for API operations during rule execution
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure of the rules operation.
    ///
    /// # Operation Flow
    ///
    /// ## Rule Loading
    /// - Attempts to load existing rules from configuration file
    /// - Creates default configuration if no rules file exists
    /// - Validates rule structure and consistency
    ///
    /// ## Subcommand Dispatch
    /// - **Config operations**: Delegate to ConfigCli for rule management
    /// - **Run operations**: Delegate to RunCli for rule execution
    ///
    /// # Error Handling
    ///
    /// The method handles several error conditions:
    /// - **Configuration errors**: Problems loading or parsing rules file
    /// - **Validation errors**: Invalid rule configurations or conflicts
    /// - **Execution errors**: Failures during rule processing or Gmail operations
    ///
    /// # Side Effects
    ///
    /// ## Configuration Mode
    /// - May modify the rules configuration file
    /// - Creates backup copies of configuration when making changes
    /// - Validates configuration consistency after modifications
    ///
    /// ## Execution Mode
    /// - May modify Gmail messages according to rule actions
    /// - Produces detailed logging of operations performed
    /// - Updates rule execution tracking and statistics
    ///
    /// # Safety Features
    ///
    /// - **Configuration validation**: Rules are validated before use
    /// - **Error isolation**: Subcommand errors don't affect rule loading
    /// - **State preservation**: Configuration errors don't corrupt existing rules
    pub async fn run(&self, client: &mut GmailClient) -> Result<()> {
        self.run_with_rules_path(client, None).await
    }

    /// Executes the rules command with an optional custom rules path.
    ///
    /// # Arguments
    ///
    /// * `client` - Mutable Gmail client for API operations
    /// * `rules_path` - Optional path to rules file
    pub async fn run_with_rules_path(
        &self,
        client: &mut GmailClient,
        rules_path: Option<&Path>,
    ) -> Result<()> {
        let rules = get_rules_from(rules_path)?;

        match &self.sub_command {
            SubCmds::Config(config_cli) => config_cli.run(rules),
            SubCmds::Run(run_cli) => run_cli.run(client, rules).await,
        }
    }
}

/// Loads Gmail retention rules from configuration with automatic fallback.
///
/// This function provides robust rule loading with automatic configuration
/// creation when no existing rules are found. It ensures that the rules
/// subsystem always has a valid configuration to work with.
///
/// # Returns
///
/// Returns `Result<Rules>` containing the loaded or newly created rules configuration.
///
/// # Loading Strategy
///
/// ## Primary Path
/// - Attempts to load existing rules from the configured rules file
/// - Validates rule structure and consistency
/// - Returns loaded rules if successful
///
/// ## Fallback Path
/// - Creates new default rules configuration if loading fails
/// - Saves the default configuration to disk for future use
/// - Returns the newly created default configuration
///
/// # Configuration Location
///
/// Rules are typically stored in:
/// - **Default location**: `~/.cull-gmail/rules.toml`
/// - **Format**: TOML configuration with structured rule definitions
/// - **Permissions**: Should be readable/writable by user only
///
/// # Error Handling
///
/// The function handles various error scenarios:
/// - **Missing configuration**: Creates default configuration automatically
/// - **Corrupted configuration**: Logs warnings and falls back to defaults
/// - **File system errors**: Propagates errors for disk access issues
///
/// # Default Configuration
///
/// When creating a new configuration, the function:
/// - Initializes an empty rules collection
/// - Sets up proper TOML structure for future rule additions
/// - Saves the configuration to disk for persistence
///
/// # Logging
///
/// The function provides appropriate logging:
/// - **Info**: Successful rule loading
/// - **Warn**: Fallback to default configuration
/// - **Error**: Critical failures during configuration creation
///
/// # Usage Context
///
/// This function is called by:
/// - **Rules CLI**: To load rules before configuration or execution
/// - **Main CLI**: For default rule execution when no subcommand is specified
/// - **Validation systems**: To verify rule configuration integrity
///
/// Loads rules from the default location.
pub fn get_rules() -> Result<Rules> {
    get_rules_from(None)
}

/// Loads rules from a specified path, or the default location if None.
///
/// # Arguments
///
/// * `path` - Optional path to the rules file
///
/// # Returns
///
/// Returns the loaded rules, or creates and saves default rules if not found.
pub fn get_rules_from(path: Option<&Path>) -> Result<Rules> {
    match Rules::load_from(path) {
        Ok(c) => Ok(c),
        Err(_) => {
            log::warn!("Configuration not found, creating default config.");
            let rules = Rules::new();
            rules.save_to(path)?;
            Ok(rules)
        }
    }
}
