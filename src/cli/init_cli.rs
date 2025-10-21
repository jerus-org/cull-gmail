//! # Initialization CLI Module
//!
//! This module provides CLI functionality for initializing the cull-gmail application,
//! including creating configuration directories, setting up OAuth2 credentials,
//! generating default configuration files, and completing the initial authentication flow.
//!
//! ## Overview
//!
//! The initialization system allows users to:
//!
//! - **Create configuration directory**: Set up the cull-gmail configuration directory
//! - **Install credentials**: Copy and validate OAuth2 credential files  
//! - **Generate configuration**: Create default cull-gmail.toml and rules.toml files
//! - **Complete OAuth2 flow**: Authenticate with Gmail API and persist tokens
//! - **Interactive setup**: Guide users through setup with prompts and confirmations
//! - **Dry-run mode**: Preview all actions without making changes
//!
//! ## Use Cases
//!
//! ### First-time Setup
//! ```bash
//! # Interactive setup with credential file
//! cull-gmail init --interactive --credential-file ~/Downloads/client_secret.json
//!
//! # Non-interactive setup (credential file copied manually later)
//! cull-gmail init --config-dir ~/.cull-gmail
//! ```
//!
//! ### Planning and Verification
//! ```bash
//! # See what would be created without making changes
//! cull-gmail init --dry-run
//!
//! # Preview with specific options
//! cull-gmail init --config-dir /custom/path --credential-file credentials.json --dry-run
//! ```
//!
//! ### Force Overwrite
//! ```bash
//! # Recreate configuration, backing up existing files
//! cull-gmail init --force
//! ```
//!
//! ## Security Considerations
//!
//! - **Credential Protection**: OAuth2 credential files are copied with 0600 permissions
//! - **Token Directory**: Token cache directory is created with 0700 permissions
//! - **Backup Safety**: Existing files are backed up with timestamps before overwriting
//! - **Interactive Confirmation**: Prompts for confirmation before overwriting existing files

use chrono::Local;
use clap::Parser;
use dialoguer::{Confirm, Input};
use google_gmail1::yup_oauth2::ConsoleApplicationSecret;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::{env, fs};

use cull_gmail::{ClientConfig, Error, GmailClient, Result};
use lazy_regex::{Lazy, Regex, lazy_regex};

/// Parse configuration root path with h:, c:, r: prefixes.
fn parse_config_root(path_str: &str) -> PathBuf {
    static ROOT_CONFIG: Lazy<Regex> = lazy_regex!(r"^(?P<class>[hrc]):(?P<path>.+)$");

    if let Some(captures) = ROOT_CONFIG.captures(path_str) {
        let path_part = captures.name("path").map_or("", |m| m.as_str());
        let class = captures.name("class").map_or("", |m| m.as_str());

        match class {
            "h" => env::home_dir().unwrap_or_default().join(path_part),
            "c" => env::current_dir().unwrap_or_default().join(path_part),
            "r" => PathBuf::from("/").join(path_part),
            _ => PathBuf::from(path_str),
        }
    } else {
        PathBuf::from(path_str)
    }
}

/// Initialize cull-gmail configuration, credentials, and OAuth2 tokens.
///
/// This command sets up the complete cull-gmail environment by creating the configuration
/// directory, installing OAuth2 credentials, generating default configuration files,
/// and completing the initial Gmail API authentication to persist tokens.
///
/// ## Setup Process
///
/// 1. **Configuration Directory**: Create or verify the configuration directory
/// 2. **Credential Installation**: Copy and validate OAuth2 credential file (if provided)  
/// 3. **Configuration Generation**: Create cull-gmail.toml with safe defaults
/// 4. **Rules Template**: Generate rules.toml with example retention rules
/// 5. **Token Directory**: Ensure OAuth2 token cache directory exists
/// 6. **Authentication**: Complete OAuth2 flow to generate and persist tokens
///
/// ## Interactive vs Non-Interactive
///
/// - **Non-interactive** (default): Proceeds with provided options, fails if conflicts exist
/// - **Interactive** (`--interactive`): Prompts for missing information and confirmation for conflicts
/// - **Dry-run** (`--dry-run`): Shows planned actions without making any changes
///
/// ## Examples
///
/// ```bash
/// # Basic initialization
/// cull-gmail init
///
/// # Interactive setup with credential file
/// cull-gmail init --interactive --credential-file client_secret.json
///
/// # Custom configuration directory
/// cull-gmail init --config-dir /path/to/config
///
/// # Preview actions without changes
/// cull-gmail init --dry-run
///
/// # Force overwrite existing files
/// cull-gmail init --force
/// ```
#[derive(Parser, Debug)]
pub struct InitCli {
    /// Configuration directory path.
    ///
    /// Supports path prefixes:
    /// - `h:path` - Relative to home directory (default: `h:.cull-gmail`)
    /// - `c:path` - Relative to current directory
    /// - `r:path` - Relative to filesystem root
    /// - `path` - Use path as-is
    #[arg(
        long = "config-dir",
        value_name = "DIR",
        default_value = "h:.cull-gmail",
        help = "Configuration directory path"
    )]
    pub config_dir: String,

    /// OAuth2 credential file path.
    ///
    /// This should be the JSON file downloaded from Google Cloud Console
    /// containing your OAuth2 client credentials for Desktop application type.
    /// The file will be copied to the configuration directory as `credential.json`.
    #[arg(
        long = "credential-file",
        value_name = "PATH",
        help = "Path to OAuth2 credential JSON file"
    )]
    pub credential_file: Option<PathBuf>,

    /// Overwrite existing files without prompting.
    ///
    /// When enabled, existing configuration files will be backed up with
    /// timestamps and then overwritten with new defaults. Use with caution
    /// as this will replace your current configuration.
    #[arg(
        long = "force",
        help = "Overwrite existing files (creates timestamped backups)"
    )]
    pub force: bool,

    /// Show planned actions without making changes.
    ///
    /// Enables preview mode where all planned operations are displayed
    /// but no files are created, modified, or removed. OAuth2 authentication
    /// flow is also skipped in dry-run mode.
    #[arg(long = "dry-run", help = "Preview actions without making changes")]
    pub dry_run: bool,

    /// Enable interactive prompts and confirmations.
    ///
    /// When enabled, the command will prompt for missing information
    /// (such as credential file path) and ask for confirmation before
    /// overwriting existing files. Recommended for first-time users.
    #[arg(
        long = "interactive",
        short = 'i',
        help = "Prompt for missing information and confirmations"
    )]
    pub interactive: bool,
}

/// Operations that can be performed during initialization.
///
/// Each operation represents a discrete action that needs to be taken
/// to set up the cull-gmail environment. Operations are planned first
/// and then executed in the correct order with appropriate error handling.
#[derive(Debug, Clone)]
enum Operation {
    /// Create a directory with specified permissions.
    CreateDir {
        path: PathBuf,
        #[cfg(unix)]
        mode: Option<u32>,
    },

    /// Copy a file from source to destination with optional chmod and backup.
    CopyFile {
        from: PathBuf,
        to: PathBuf,
        #[cfg(unix)]
        mode: Option<u32>,
        backup_if_exists: bool,
    },

    /// Write content to a file with optional permissions and backup.
    WriteFile {
        path: PathBuf,
        contents: String,
        #[cfg(unix)]
        mode: Option<u32>,
        backup_if_exists: bool,
    },

    /// Ensure token directory exists with secure permissions.
    EnsureTokenDir {
        path: PathBuf,
        #[cfg(unix)]
        mode: Option<u32>,
    },

    /// Run OAuth2 authentication flow.
    RunOAuth2 {
        config_root: String,
        credential_file: Option<String>,
    },
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::CreateDir { path, .. } => {
                write!(f, "Create directory: {}", path.display())
            }
            Operation::CopyFile {
                from,
                to,
                backup_if_exists,
                ..
            } => {
                if *backup_if_exists && to.exists() {
                    write!(
                        f,
                        "Copy file: {} → {} (with backup)",
                        from.display(),
                        to.display()
                    )
                } else {
                    write!(f, "Copy file: {} → {}", from.display(), to.display())
                }
            }
            Operation::WriteFile {
                path,
                backup_if_exists,
                ..
            } => {
                if *backup_if_exists && path.exists() {
                    write!(f, "Write file: {} (with backup)", path.display())
                } else {
                    write!(f, "Write file: {}", path.display())
                }
            }
            Operation::EnsureTokenDir { path, .. } => {
                write!(f, "Ensure token directory: {}", path.display())
            }
            Operation::RunOAuth2 { .. } => {
                write!(f, "Run OAuth2 authentication flow")
            }
        }
    }
}

/// Configuration defaults for initialization.
struct InitDefaults;

impl InitDefaults {
    const CONFIG_FILE_CONTENT: &'static str = r#"# cull-gmail configuration
# This file configures the cull-gmail application.

# OAuth2 credential file (relative to config_root)
credential_file = "credential.json"

# Configuration root directory  
config_root = "h:.cull-gmail"

# Rules configuration file
rules = "rules.toml"

# Default execution mode (false = dry-run, true = execute)
# Set to false for safety - you can override with --execute flag
execute = false

# Environment variable name for token cache (for ephemeral environments)
token_cache_env = "CULL_GMAIL_TOKEN_CACHE"
"#;

    const RULES_FILE_CONTENT: &'static str = r#"# Example rules for cull-gmail
# Each rule targets a Gmail label and specifies an action.
# 
# Actions:
#   - "Trash" is recoverable (messages go to Trash folder ~30 days)
#   - "Delete" is irreversible (messages are permanently deleted)
#
# Time formats:
#   - "older_than:30d" (30 days)
#   - "older_than:6m" (6 months) 
#   - "older_than:2y" (2 years)
#
# Example rule for promotional emails:
# [[rules]]
# id = 1
# label = "Promotions"
# query = "category:promotions older_than:30d"
# action = "Trash"
#
# Example rule for old newsletters:
# [[rules]]
# id = 2
# label = "Updates"
# query = "category:updates older_than:90d"
# action = "Trash"
#
# Uncomment and modify the examples above to create your own rules.
# Run 'cull-gmail rules run --dry-run' to test rules before execution.
"#;

    fn credential_filename() -> &'static str {
        "credential.json"
    }

    fn config_filename() -> &'static str {
        "cull-gmail.toml"
    }

    fn rules_filename() -> &'static str {
        "rules.toml"
    }

    fn token_dir_name() -> &'static str {
        "gmail1"
    }
}

impl InitCli {
    /// Execute the initialization command.
    ///
    /// This method orchestrates the complete initialization workflow including
    /// configuration directory creation, credential installation, file generation,
    /// and OAuth2 authentication based on the provided command-line options.
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure of the initialization process.
    ///
    /// # Process Flow
    ///
    /// 1. **Plan Operations**: Analyze current state and generate operation plan
    /// 2. **Validate Inputs**: Check credential file validity and resolve paths  
    /// 3. **Interactive Prompts**: Request missing information if in interactive mode
    /// 4. **Execute or Preview**: Apply operations or show dry-run preview
    /// 5. **OAuth2 Flow**: Complete authentication and token generation
    /// 6. **Success Reporting**: Display results and next steps
    ///
    /// # Errors
    ///
    /// This method can return errors for:
    /// - Invalid or missing credential files
    /// - File system permission issues  
    /// - Configuration conflicts without force or interactive resolution
    /// - OAuth2 authentication failures
    /// - Network connectivity issues during authentication
    pub async fn run(&self) -> Result<()> {
        log::info!("Starting cull-gmail initialization");

        if self.dry_run {
            println!("🔍 DRY RUN: No changes will be made\n");
        }

        // Resolve configuration directory path
        let config_path = parse_config_root(&self.config_dir);

        log::info!("Configuration directory: {}", config_path.display());

        // Handle interactive credential file prompt if needed
        let credential_file = self.get_credential_file().await?;

        // Plan all operations
        let operations = self.plan_operations(&config_path, credential_file.as_ref())?;

        // Show plan in dry-run mode
        if self.dry_run {
            self.show_plan(&operations);
            return Ok(());
        }

        // Execute operations
        self.execute_operations(&operations).await?;

        // Show success message and next steps
        self.show_completion(&config_path);

        Ok(())
    }

    /// Get credential file path, prompting if interactive and not provided.
    async fn get_credential_file(&self) -> Result<Option<PathBuf>> {
        if let Some(ref cred_file) = self.credential_file {
            // Validate the provided credential file
            self.validate_credential_file(cred_file)?;
            return Ok(Some(cred_file.clone()));
        }

        if self.interactive {
            println!("📋 OAuth2 credential file setup");
            println!("You need a credential JSON file from Google Cloud Console.");
            println!("Visit: https://console.cloud.google.com/apis/credentials\n");

            let should_provide = Confirm::new()
                .with_prompt("Do you have a credential file to set up now?")
                .default(true)
                .interact()
                .map_err(|e| Error::FileIo(format!("Interactive prompt failed: {e}")))?;

            if should_provide {
                let cred_path: String = Input::new()
                    .with_prompt("Path to credential JSON file")
                    .interact_text()
                    .map_err(|e| Error::FileIo(format!("Interactive input failed: {e}")))?;

                let cred_file = PathBuf::from(cred_path);
                self.validate_credential_file(&cred_file)?;
                return Ok(Some(cred_file));
            } else {
                println!("⏭️  Skipping credential setup - you can add it later\n");
            }
        }

        Ok(None)
    }

    /// Validate that a credential file exists and can be parsed.
    fn validate_credential_file(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Err(Error::FileIo(format!(
                "Credential file not found: {}",
                path.display()
            )));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| Error::FileIo(format!("Cannot read credential file: {e}")))?;

        // Try to parse as ConsoleApplicationSecret to validate format
        serde_json::from_str::<ConsoleApplicationSecret>(&content).map_err(|e| {
            Error::SerializationError(format!("Invalid credential file format: {e}"))
        })?;

        log::info!("Credential file validated: {}", path.display());
        Ok(())
    }

    /// Plan all operations needed for initialization.
    fn plan_operations(
        &self,
        config_path: &Path,
        credential_file: Option<&PathBuf>,
    ) -> Result<Vec<Operation>> {
        let mut operations = Vec::new();

        // 1. Create config directory if it doesn't exist
        self.plan_config_directory(&mut operations, config_path);

        // 2. Copy credential file if provided
        if let Some(cred_file) = credential_file {
            self.plan_credential_file_operation(&mut operations, config_path, cred_file)?;
        }

        // 3. Write config file
        self.plan_config_file_operation(&mut operations, config_path)?;

        // 4. Write rules file
        self.plan_rules_file_operation(&mut operations, config_path)?;

        // 5. Ensure token directory exists
        self.plan_token_directory(&mut operations, config_path);

        // 6. Run OAuth2 if we have credentials
        if credential_file.is_some() {
            self.plan_oauth_operation(&mut operations);
        }

        Ok(operations)
    }

    /// Plan config directory creation.
    fn plan_config_directory(&self, operations: &mut Vec<Operation>, config_path: &Path) {
        if !config_path.exists() {
            operations.push(Operation::CreateDir {
                path: config_path.to_path_buf(),
                #[cfg(unix)]
                mode: Some(0o755),
            });
        }
    }

    /// Plan credential file copy operation.
    fn plan_credential_file_operation(
        &self,
        operations: &mut Vec<Operation>,
        config_path: &Path,
        cred_file: &PathBuf,
    ) -> Result<()> {
        let dest_path = config_path.join(InitDefaults::credential_filename());
        self.check_file_conflicts(&dest_path, "Credential file")?;

        operations.push(Operation::CopyFile {
            from: cred_file.clone(),
            to: dest_path.clone(),
            #[cfg(unix)]
            mode: Some(0o600),
            backup_if_exists: self.should_backup(&dest_path),
        });
        Ok(())
    }

    /// Plan config file write operation.
    fn plan_config_file_operation(
        &self,
        operations: &mut Vec<Operation>,
        config_path: &Path,
    ) -> Result<()> {
        let config_file_path = config_path.join(InitDefaults::config_filename());
        self.check_file_conflicts(&config_file_path, "Configuration file")?;

        operations.push(Operation::WriteFile {
            path: config_file_path.clone(),
            contents: InitDefaults::CONFIG_FILE_CONTENT.to_string(),
            #[cfg(unix)]
            mode: Some(0o644),
            backup_if_exists: self.should_backup(&config_file_path),
        });
        Ok(())
    }

    /// Plan rules file write operation.
    fn plan_rules_file_operation(
        &self,
        operations: &mut Vec<Operation>,
        config_path: &Path,
    ) -> Result<()> {
        let rules_file_path = config_path.join(InitDefaults::rules_filename());
        self.check_file_conflicts(&rules_file_path, "Rules file")?;

        operations.push(Operation::WriteFile {
            path: rules_file_path.clone(),
            contents: InitDefaults::RULES_FILE_CONTENT.to_string(),
            #[cfg(unix)]
            mode: Some(0o644),
            backup_if_exists: self.should_backup(&rules_file_path),
        });
        Ok(())
    }

    /// Plan token directory creation.
    fn plan_token_directory(&self, operations: &mut Vec<Operation>, config_path: &Path) {
        let token_dir = config_path.join(InitDefaults::token_dir_name());
        operations.push(Operation::EnsureTokenDir {
            path: token_dir,
            #[cfg(unix)]
            mode: Some(0o700),
        });
    }

    /// Plan OAuth2 operation.
    fn plan_oauth_operation(&self, operations: &mut Vec<Operation>) {
        operations.push(Operation::RunOAuth2 {
            config_root: self.config_dir.clone(),
            credential_file: Some(InitDefaults::credential_filename().to_string()),
        });
    }

    /// Check for file conflicts and return appropriate error if needed.
    fn check_file_conflicts(&self, file_path: &Path, file_type: &str) -> Result<()> {
        if file_path.exists() && !self.force && !self.interactive {
            return Err(Error::FileIo(format!(
                "{} already exists: {}\nUse --force to overwrite or --interactive for prompts",
                file_type,
                file_path.display()
            )));
        }
        Ok(())
    }

    /// Determine if a file should be backed up.
    fn should_backup(&self, file_path: &Path) -> bool {
        file_path.exists() && self.force
    }

    /// Show the planned operations in dry-run mode.
    fn show_plan(&self, operations: &[Operation]) {
        println!("📋 Planned operations:");
        for (i, op) in operations.iter().enumerate() {
            println!("  {}. {}", i + 1, op);
        }
        println!();

        if operations
            .iter()
            .any(|op| matches!(op, Operation::RunOAuth2 { .. }))
        {
            println!("🔐 OAuth2 authentication would open your browser for Gmail authorization");
        } else {
            println!("⚠️  OAuth2 authentication skipped - no credential file provided");
            println!("   Add a credential file later and run 'cull-gmail init' again");
        }

        println!();
        println!("To apply these changes, run without --dry-run");
    }

    /// Execute all planned operations.
    async fn execute_operations(&self, operations: &[Operation]) -> Result<()> {
        let pb = ProgressBar::new(operations.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        for (i, operation) in operations.iter().enumerate() {
            pb.set_position(i as u64);
            pb.set_message(format!("{operation}"));

            self.execute_operation(operation).await?;

            // Small delay to make progress visible
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        pb.finish_with_message("✅ All operations completed");
        println!();

        Ok(())
    }

    /// Execute a single operation.
    async fn execute_operation(&self, operation: &Operation) -> Result<()> {
        match operation {
            Operation::CreateDir { path, .. } => {
                self.execute_create_directory(path, operation).await
            }
            Operation::CopyFile {
                from,
                to,
                backup_if_exists,
                ..
            } => {
                self.execute_copy_file(from, to, *backup_if_exists, operation)
                    .await
            }
            Operation::WriteFile {
                path,
                contents,
                backup_if_exists,
                ..
            } => {
                self.execute_write_file(path, contents, *backup_if_exists, operation)
                    .await
            }
            Operation::EnsureTokenDir { path, .. } => {
                self.execute_ensure_token_directory(path, operation).await
            }
            Operation::RunOAuth2 {
                config_root,
                credential_file,
            } => {
                self.execute_oauth_flow(config_root, credential_file).await
            }
        }
    }

    /// Execute directory creation operation.
    async fn execute_create_directory(&self, path: &Path, operation: &Operation) -> Result<()> {
        log::info!("Creating directory: {}", path.display());
        fs::create_dir_all(path)
            .map_err(|e| Error::FileIo(format!("Failed to create directory: {e}")))?;

        self.apply_permissions_if_needed(path, operation)
    }

    /// Execute file copy operation.
    async fn execute_copy_file(
        &self,
        from: &Path,
        to: &Path,
        backup_if_exists: bool,
        operation: &Operation,
    ) -> Result<()> {
        self.handle_existing_file(to, backup_if_exists, "file copy").await?;

        log::info!("Copying file: {} → {}", from.display(), to.display());
        fs::copy(from, to).map_err(|e| Error::FileIo(format!("Failed to copy file: {e}")))?;

        self.apply_permissions_if_needed(to, operation)
    }

    /// Execute file write operation.
    async fn execute_write_file(
        &self,
        path: &Path,
        contents: &str,
        backup_if_exists: bool,
        operation: &Operation,
    ) -> Result<()> {
        self.handle_existing_file(path, backup_if_exists, "file write").await?;

        log::info!("Writing file: {}", path.display());
        fs::write(path, contents).map_err(|e| Error::FileIo(format!("Failed to write file: {e}")))?;

        self.apply_permissions_if_needed(path, operation)
    }

    /// Execute token directory creation operation.
    async fn execute_ensure_token_directory(&self, path: &Path, operation: &Operation) -> Result<()> {
        log::info!("Ensuring token directory: {}", path.display());
        fs::create_dir_all(path)
            .map_err(|e| Error::FileIo(format!("Failed to create token directory: {e}")))?;

        self.apply_permissions_if_needed(path, operation)
    }

    /// Execute OAuth2 authentication flow.
    async fn execute_oauth_flow(
        &self,
        config_root: &str,
        credential_file: &Option<String>,
    ) -> Result<()> {
        if credential_file.is_some() {
            log::info!("Starting OAuth2 authentication flow");
            self.run_oauth_flow(config_root).await
        } else {
            log::warn!("Skipping OAuth2 - no credential file available");
            Ok(())
        }
    }

    /// Handle existing file logic (backup or interactive prompt).
    async fn handle_existing_file(
        &self,
        path: &Path,
        backup_if_exists: bool,
        operation_name: &str,
    ) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }

        if backup_if_exists {
            self.create_backup(path)
        } else if self.interactive {
            self.prompt_for_overwrite(path, operation_name).await
        } else {
            Ok(())
        }
    }

    /// Prompt user for file overwrite confirmation.
    async fn prompt_for_overwrite(&self, path: &Path, operation_name: &str) -> Result<()> {
        let should_overwrite = Confirm::new()
            .with_prompt(format!("Overwrite existing file {}?", path.display()))
            .default(false)
            .interact()
            .map_err(|e| Error::FileIo(format!("Interactive prompt failed: {e}")))?;

        if !should_overwrite {
            log::info!("Skipping {} due to user choice", operation_name);
            return Ok(());
        }

        self.create_backup(path)
    }

    /// Apply file permissions if needed (Unix only).
    fn apply_permissions_if_needed(&self, path: &Path, operation: &Operation) -> Result<()> {
        #[cfg(unix)]
        if let Some(mode) = operation.get_mode() {
            self.set_permissions(path, mode)?;
        }
        Ok(())
    }

    /// Create a timestamped backup of a file.
    fn create_backup(&self, file_path: &Path) -> Result<()> {
        let timestamp = Local::now().format("%Y%m%d%H%M%S");
        let backup_path = file_path.with_extension(format!("bak-{timestamp}"));

        log::info!(
            "Creating backup: {} → {}",
            file_path.display(),
            backup_path.display()
        );
        fs::copy(file_path, &backup_path)
            .map_err(|e| Error::FileIo(format!("Failed to create backup: {e}")))?;

        Ok(())
    }

    /// Set file permissions (Unix only).
    #[cfg(unix)]
    fn set_permissions(&self, path: &Path, mode: u32) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let metadata = fs::metadata(path)
            .map_err(|e| Error::FileIo(format!("Failed to get file metadata: {e}")))?;

        let mut permissions = metadata.permissions();
        permissions.set_mode(mode);

        fs::set_permissions(path, permissions)
            .map_err(|e| Error::FileIo(format!("Failed to set file permissions: {e}")))?;

        log::debug!("Set permissions {:o} on {}", mode, path.display());
        Ok(())
    }

    /// Run OAuth2 authentication flow.
    async fn run_oauth_flow(&self, config_root: &str) -> Result<()> {
        println!("🔐 Starting OAuth2 authentication...");
        println!("This will open your web browser for Gmail authorization.");

        // Parse config root and build ClientConfig
        let config_path = parse_config_root(config_root);

        let client_config = ClientConfig::builder()
            .with_credential_file(InitDefaults::credential_filename())
            .with_config_path(config_path.to_string_lossy().as_ref())
            .build();

        // Initialize Gmail client which will trigger OAuth flow if needed
        let client = GmailClient::new_with_config(client_config)
            .await
            .map_err(|e| Error::FileIo(format!("OAuth2 authentication failed: {e}")))?;

        // The client initialization already verified the connection by fetching labels
        // We can just show some labels to confirm it's working
        client.show_label();
        println!("✅ OAuth2 authentication successful!");
        log::info!("OAuth2 tokens generated and cached");

        Ok(())
    }

    /// Show completion message and next steps.
    fn show_completion(&self, config_path: &Path) {
        println!("🎉 Initialization completed successfully!\n");

        println!("📁 Configuration directory: {}", config_path.display());
        println!("📄 Files created:");
        println!("   - cull-gmail.toml (main configuration)");
        println!("   - rules.toml (retention rules template)");
        if self.credential_file.is_some() {
            println!("   - credential.json (OAuth2 credentials)");
            println!("   - gmail1/ (OAuth2 token cache)");
        }
        println!();

        println!("📋 Next steps:");
        if self.credential_file.is_some() {
            println!("   1. Test Gmail connection: cull-gmail labels");
            println!("   2. Review rules template: cull-gmail rules run --dry-run");
            println!("   3. Customize rules.toml as needed");
            println!("   4. Run rules safely: cull-gmail rules run --dry-run");
            println!("   5. Execute for real: cull-gmail rules run --execute");
        } else {
            println!("   1. Add your OAuth2 credential file to:");
            println!("      {}/credential.json", config_path.display());
            println!("   2. Complete setup: cull-gmail init");
            println!("   3. Or get credentials from:");
            println!("      https://console.cloud.google.com/apis/credentials");
        }
        println!();

        println!("💡 Tips:");
        println!("   - All operations use dry-run mode by default for safety");
        println!("   - Use --execute flag or set execute=true in config for real actions");
        println!("   - See 'cull-gmail --help' for all available commands");
    }
}

impl Operation {
    /// Get the file mode for this operation (Unix only).
    #[cfg(unix)]
    fn get_mode(&self) -> Option<u32> {
        match self {
            Operation::CreateDir { mode, .. }
            | Operation::CopyFile { mode, .. }
            | Operation::WriteFile { mode, .. }
            | Operation::EnsureTokenDir { mode, .. } => *mode,
            Operation::RunOAuth2 { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests;
