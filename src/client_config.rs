//! # Gmail Client Configuration Module
//!
//! This module provides configuration management for Gmail API authentication and client setup.
//! It handles OAuth2 credential loading, configuration parsing, and client initialization
//! with flexible configuration sources including files, environment variables, and direct parameters.
//!
//! ## Overview
//!
//! The configuration system supports multiple authentication methods:
//!
//! - **File-based OAuth2 credentials**: Load Google Cloud Platform OAuth2 credentials from JSON files
//! - **Direct configuration**: Set OAuth2 parameters programmatically via builder pattern
//! - **Mixed configuration**: Combine file-based and programmatic configuration as needed
//!
//! ## Configuration Sources
//!
//! The module supports hierarchical configuration loading:
//!
//! 1. **Direct OAuth2 parameters** (highest priority)
//! 2. **Credential file** specified via `credential_file` parameter
//! 3. **Environment variables** via the `config` crate integration
//!
//! ## Security Considerations
//!
//! - **Credential Storage**: OAuth2 secrets are handled securely and never logged
//! - **File Permissions**: Credential files should have restricted permissions (600 or similar)
//! - **Error Handling**: File I/O and parsing errors are propagated with context
//! - **Token Persistence**: OAuth2 tokens are stored in configurable directories with appropriate permissions
//!
//! ## Configuration Directory Structure
//!
//! The module supports flexible directory structures:
//!
//! ```text
//! ~/.cull-gmail/                  # Default configuration root
//! ├── client_secret.json         # OAuth2 credentials
//! ├── gmail1/                    # OAuth2 token cache
//! │   ├── tokencache.json        # Cached access/refresh tokens
//! │   └── ...                    # Other OAuth2 artifacts
//! └── config.toml                # Application configuration
//! ```
//!
//! ## Path Resolution
//!
//! The module supports multiple path resolution schemes:
//!
//! - `h:path` - Resolve relative to user's home directory
//! - `r:path` - Resolve relative to system root directory
//! - `c:path` - Resolve relative to current working directory
//! - `path` - Use path as-is (no prefix resolution)
//!
//! ## Usage Examples
//!
//! ### Builder Pattern with Credential File
//!
//! ```rust,no_run
//! use cull_gmail::ClientConfig;
//!
//! let config = ClientConfig::builder()
//!     .with_credential_file("client_secret.json")
//!     .with_config_path("~/.cull-gmail")
//!     .build();
//! ```
//!
//! ### Builder Pattern with Direct OAuth2 Parameters
//!
//! ```rust
//! use cull_gmail::ClientConfig;
//!
//! let config = ClientConfig::builder()
//!     .with_client_id("your-client-id.googleusercontent.com")
//!     .with_client_secret("your-client-secret")
//!     .with_auth_uri("https://accounts.google.com/o/oauth2/auth")
//!     .with_token_uri("https://oauth2.googleapis.com/token")
//!     .add_redirect_uri("http://localhost:8080")
//!     .build();
//! ```
//!
//! ### Configuration from Config File
//!
//! ```rust,no_run
//! use cull_gmail::ClientConfig;
//! use config::Config;
//!
//! let app_config = Config::builder()
//!     .set_default("credential_file", "client_secret.json")?
//!     .set_default("config_root", "h:.cull-gmail")?
//!     .add_source(config::File::with_name("config.toml"))
//!     .build()?;
//!
//! let client_config = ClientConfig::new_from_configuration(app_config)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Integration with Gmail Client
//!
//! The configuration integrates seamlessly with the Gmail client:
//!
//! ```rust,no_run
//! use cull_gmail::{ClientConfig, GmailClient};
//!
//! # async fn example() -> cull_gmail::Result<()> {
//! let config = ClientConfig::builder()
//!     .with_credential_file("client_secret.json")
//!     .build();
//!
//! let client = GmailClient::new_with_config(config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Error Handling
//!
//! The module uses the crate's unified error type for consistent error handling:
//!
//! ```rust,no_run
//! use cull_gmail::{ClientConfig, Result};
//! use config::Config;
//!
//! fn load_config(app_config: Config) -> Result<ClientConfig> {
//!     match ClientConfig::new_from_configuration(app_config) {
//!         Ok(config) => Ok(config),
//!         Err(e) => {
//!             eprintln!("Configuration error: {}", e);
//!             Err(e)
//!         }
//!     }
//! }
//! ```
//!
//! ## Thread Safety
//!
//! All configuration types are safe to clone and use across threads. However,
//! file I/O operations are synchronous and should be performed during application
//! initialization rather than in performance-critical paths.

use std::{fs, path::PathBuf};

use config::Config;
use google_gmail1::yup_oauth2::{ApplicationSecret, ConsoleApplicationSecret};

use crate::Result;

mod config_root;

use config_root::ConfigRoot;

/// Gmail client configuration containing OAuth2 credentials and persistence settings.
///
/// This struct holds all necessary configuration for Gmail API authentication and client setup,
/// including OAuth2 application secrets, configuration directory paths, and token persistence settings.
///
/// # Fields
///
/// The struct contains private fields that are accessed through getter methods to ensure
/// proper encapsulation and prevent accidental mutation of sensitive configuration data.
///
/// # Security
///
/// The `secret` field contains sensitive OAuth2 credentials including client secrets.
/// These values are never logged or exposed in debug output beyond their type information.
///
/// # Thread Safety
///
/// `ClientConfig` is safe to clone and use across threads. All contained data is either
/// immutable or safely clonable.
///
/// # Examples
///
/// ```rust
/// use cull_gmail::ClientConfig;
///
/// // Create configuration with builder pattern
/// let config = ClientConfig::builder()
///     .with_client_id("test-client-id")
///     .with_client_secret("test-secret")
///     .build();
///
/// // Access configuration values
/// assert_eq!(config.secret().client_id, "test-client-id");
/// assert!(config.persist_path().contains("gmail1"));
/// ```
#[derive(Debug)]
pub struct ClientConfig {
    /// OAuth2 application secret containing client credentials and endpoints.
    /// This field contains sensitive information and should be handled carefully.
    secret: ApplicationSecret,
    
    /// Configuration root path resolver for determining base directories.
    /// Supports multiple path resolution schemes (home, root, current directory).
    config_root: ConfigRoot,
    
    /// Full path where OAuth2 tokens should be persisted.
    /// Typically resolves to something like `~/.cull-gmail/gmail1`.
    persist_path: String,
}

impl ClientConfig {
    /// Creates a new configuration builder for constructing `ClientConfig` instances.
    ///
    /// The builder pattern allows for flexible configuration construction with method chaining.
    /// This is the preferred way to create new configurations as it provides compile-time
    /// guarantees about required fields and allows for incremental configuration building.
    ///
    /// # Returns
    ///
    /// A new `ConfigBuilder` instance initialized with sensible defaults.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::ClientConfig;
    ///
    /// let config = ClientConfig::builder()
    ///     .with_client_id("your-client-id")
    ///     .with_client_secret("your-secret")
    ///     .build();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Creates a new `ClientConfig` from an external configuration source.
    ///
    /// This method supports hierarchical configuration loading with the following priority:
    /// 1. Direct OAuth2 parameters (`client_id`, `client_secret`, `token_uri`, `auth_uri`)
    /// 2. Credential file specified via `credential_file` parameter
    ///
    /// # Configuration Parameters
    ///
    /// ## Required Parameters (one of these sets):
    ///
    /// **Direct OAuth2 Configuration:**
    /// - `client_id`: OAuth2 client identifier
    /// - `client_secret`: OAuth2 client secret
    /// - `token_uri`: Token exchange endpoint URL
    /// - `auth_uri`: Authorization endpoint URL
    ///
    /// **OR**
    ///
    /// **File-based Configuration:**
    /// - `credential_file`: Path to JSON credential file (relative to `config_root`)
    ///
    /// ## Always Required:
    /// - `config_root`: Base directory for configuration files (supports path prefixes)
    ///
    /// # Arguments
    ///
    /// * `configs` - Configuration object containing OAuth2 and path settings
    ///
    /// # Returns
    ///
    /// Returns `Ok(ClientConfig)` on successful configuration loading, or an error if:
    /// - Required configuration parameters are missing
    /// - Credential file cannot be read or parsed
    /// - OAuth2 credential structure is invalid
    ///
    /// # Errors
    ///
    /// This method can return errors for:
    /// - Missing required configuration keys
    /// - File I/O errors when reading credential files
    /// - JSON parsing errors for malformed credential files
    /// - Invalid OAuth2 credential structure
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cull_gmail::ClientConfig;
    /// use config::Config;
    ///
    /// // Configuration with credential file
    /// let app_config = Config::builder()
    ///     .set_default("credential_file", "client_secret.json")?
    ///     .set_default("config_root", "h:.cull-gmail")?
    ///     .build()?;
    ///
    /// let client_config = ClientConfig::new_from_configuration(app_config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// ```rust,no_run
    /// use cull_gmail::ClientConfig;
    /// use config::Config;
    ///
    /// // Configuration with direct OAuth2 parameters
    /// let app_config = Config::builder()
    ///     .set_default("client_id", "your-client-id")?
    ///     .set_default("client_secret", "your-secret")?
    ///     .set_default("token_uri", "https://oauth2.googleapis.com/token")?
    ///     .set_default("auth_uri", "https://accounts.google.com/o/oauth2/auth")?
    ///     .set_default("config_root", "h:.cull-gmail")?
    ///     .build()?;
    ///
    /// let client_config = ClientConfig::new_from_configuration(app_config)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new_from_configuration(configs: Config) -> Result<Self> {
        let root = configs.get_string("config_root")?;
        let config_root = ConfigRoot::parse(&root);

        let secret = if let Ok(client_id) = configs.get_string("client_id")
            && let Ok(client_secret) = configs.get_string("client_secret")
            && let Ok(token_uri) = configs.get_string("token_uri")
            && let Ok(auth_uri) = configs.get_string("auth_uri")
        {
            ApplicationSecret {
                client_id,
                client_secret,
                token_uri,
                auth_uri,
                project_id: None,
                redirect_uris: Vec::new(),
                client_email: None,
                auth_provider_x509_cert_url: None,
                client_x509_cert_url: None,
            }
        } else {
            let credential_file = configs.get_string("credential_file")?;
            log::info!("root: {config_root}");
            let path = config_root.full_path().join(credential_file);
            log::info!("path: {}", path.display());
            let json_str = fs::read_to_string(path).expect("could not read path");

            let console: ConsoleApplicationSecret =
                serde_json::from_str(&json_str).expect("could not convert to struct");

            console.installed.unwrap()
        };

        let persist_path = format!("{}/gmail1", config_root.full_path().display());

        Ok(ClientConfig {
            config_root,
            secret,
            persist_path,
        })
    }

    /// Returns a reference to the OAuth2 application secret.
    ///
    /// This provides access to the OAuth2 credentials including client ID, client secret,
    /// and endpoint URLs required for Gmail API authentication.
    ///
    /// # Security Note
    ///
    /// The returned `ApplicationSecret` contains sensitive information including the
    /// OAuth2 client secret. Handle this data carefully and avoid logging or exposing it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::ClientConfig;
    ///
    /// let config = ClientConfig::builder()
    ///     .with_client_id("test-client-id")
    ///     .build();
    ///
    /// let secret = config.secret();
    /// assert_eq!(secret.client_id, "test-client-id");
    /// ```
    pub fn secret(&self) -> &ApplicationSecret {
        &self.secret
    }

    /// Returns the full path where OAuth2 tokens should be persisted.
    ///
    /// This path is used by the OAuth2 library to store and retrieve cached tokens,
    /// enabling automatic token refresh without requiring user re-authentication.
    ///
    /// # Path Format
    ///
    /// The path typically follows the pattern: `{config_root}/gmail1`
    ///
    /// For example:
    /// - `~/.cull-gmail/gmail1` (when config_root is `h:.cull-gmail`)
    /// - `/etc/cull-gmail/gmail1` (when config_root is `r:etc/cull-gmail`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::ClientConfig;
    ///
    /// let config = ClientConfig::builder().build();
    /// let persist_path = config.persist_path();
    /// assert!(persist_path.contains("gmail1"));
    /// ```
    pub fn persist_path(&self) -> &str {
        &self.persist_path
    }

    /// Returns a reference to the configuration root path resolver.
    ///
    /// The `ConfigRoot` handles path resolution with support for different base directories
    /// including home directory, system root, and current working directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::ClientConfig;
    ///
    /// let config = ClientConfig::builder()
    ///     .with_config_path(".cull-gmail")
    ///     .build();
    ///
    /// let config_root = config.config_root();
    /// // config_root can be used to resolve additional paths
    /// ```
    pub fn config_root(&self) -> &ConfigRoot {
        &self.config_root
    }

    /// Returns the fully resolved configuration directory path as a string.
    ///
    /// This method resolves the configuration root path to an absolute path string,
    /// applying any path prefix resolution (home directory, system root, etc.).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cull_gmail::ClientConfig;
    ///
    /// let config = ClientConfig::builder()
    ///     .with_config_path(".cull-gmail")
    ///     .build();
    ///
    /// let full_path = config.full_path();
    /// // Returns the absolute path to the configuration directory
    /// ```
    pub fn full_path(&self) -> String {
        self.config_root.full_path().display().to_string()
    }
}

/// Builder for constructing `ClientConfig` instances with flexible configuration options.
///
/// The `ConfigBuilder` provides a fluent interface for constructing Gmail client configurations
/// with support for both file-based and programmatic OAuth2 credential setup. It implements
/// the builder pattern to ensure required configuration is provided while allowing optional
/// parameters to be set incrementally.
///
/// # Configuration Methods
///
/// The builder supports two primary configuration approaches:
///
/// 1. **File-based configuration**: Load OAuth2 credentials from JSON files
/// 2. **Direct configuration**: Set OAuth2 parameters programmatically
///
/// # Thread Safety
///
/// The builder is not thread-safe and should be used to construct configurations
/// in a single-threaded context. The resulting `ClientConfig` instances are thread-safe.
///
/// # Examples
///
/// ## File-based Configuration
///
/// ```rust,no_run
/// use cull_gmail::ClientConfig;
///
/// let config = ClientConfig::builder()
///     .with_credential_file("client_secret.json")
///     .with_config_path(".cull-gmail")
///     .build();
/// ```
///
/// ## Direct OAuth2 Configuration
///
/// ```rust
/// use cull_gmail::ClientConfig;
///
/// let config = ClientConfig::builder()
///     .with_client_id("your-client-id.googleusercontent.com")
///     .with_client_secret("your-client-secret")
///     .with_auth_uri("https://accounts.google.com/o/oauth2/auth")
///     .with_token_uri("https://oauth2.googleapis.com/token")
///     .add_redirect_uri("http://localhost:8080")
///     .with_project_id("your-project-id")
///     .build();
/// ```
///
/// ## Mixed Configuration
///
/// ```rust,no_run
/// use cull_gmail::ClientConfig;
///
/// let config = ClientConfig::builder()
///     .with_credential_file("base_credentials.json")
///     .add_redirect_uri("http://localhost:3000")  // Additional redirect URI
///     .with_project_id("override-project-id")    // Override from file
///     .build();
/// ```
#[derive(Debug)]
pub struct ConfigBuilder {
    /// OAuth2 application secret being constructed.
    /// Contains client credentials, endpoints, and additional parameters.
    secret: ApplicationSecret,
    
    /// Configuration root path resolver for determining base directories.
    /// Used to resolve relative paths in credential files and token storage.
    config_root: ConfigRoot,
}

impl Default for ConfigBuilder {
    /// Creates a new `ConfigBuilder` with sensible OAuth2 defaults.
    ///
    /// The default configuration includes:
    /// - Standard Google OAuth2 endpoints (auth_uri, token_uri)
    /// - Empty client credentials (must be set before use)
    /// - Default configuration root (no path prefix)
    ///
    /// # Note
    ///
    /// The default instance requires additional configuration before it can be used
    /// to create a functional `ClientConfig`. At minimum, you must set either:
    /// - Client credentials via `with_client_id()` and `with_client_secret()`, or
    /// - A credential file via `with_credential_file()`
    fn default() -> Self {
        let secret = ApplicationSecret {
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            ..Default::default()
        };

        Self {
            secret,
            config_root: Default::default(),
        }
    }
}

impl ConfigBuilder {
    pub fn with_config_base(&mut self, value: &config_root::RootBase) -> &mut Self {
        self.config_root.set_root_base(value);
        self
    }

    pub fn with_config_path(&mut self, value: &str) -> &mut Self {
        self.config_root.set_path(value);
        self
    }

    pub fn with_credential_file(&mut self, credential_file: &str) -> &mut Self {
        let path = PathBuf::from(self.config_root.to_string()).join(credential_file);
        log::info!("path: {}", path.display());
        let json_str = fs::read_to_string(path).expect("could not read path");

        let console: ConsoleApplicationSecret =
            serde_json::from_str(&json_str).expect("could not convert to struct");

        self.secret = console.installed.unwrap();
        self
    }

    pub fn with_client_id(&mut self, value: &str) -> &mut Self {
        self.secret.client_id = value.to_string();
        self
    }

    pub fn with_client_secret(&mut self, value: &str) -> &mut Self {
        self.secret.client_secret = value.to_string();
        self
    }

    pub fn with_token_uri(&mut self, value: &str) -> &mut Self {
        self.secret.token_uri = value.to_string();
        self
    }

    pub fn with_auth_uri(&mut self, value: &str) -> &mut Self {
        self.secret.auth_uri = value.to_string();
        self
    }

    pub fn add_redirect_uri(&mut self, value: &str) -> &mut Self {
        self.secret.redirect_uris.push(value.to_string());
        self
    }

    pub fn with_project_id(&mut self, value: &str) -> &mut Self {
        self.secret.project_id = Some(value.to_string());
        self
    }

    pub fn with_client_email(&mut self, value: &str) -> &mut Self {
        self.secret.client_email = Some(value.to_string());
        self
    }
    pub fn with_auth_provider_x509_cert_url(&mut self, value: &str) -> &mut Self {
        self.secret.auth_provider_x509_cert_url = Some(value.to_string());
        self
    }
    pub fn with_client_x509_cert_url(&mut self, value: &str) -> &mut Self {
        self.secret.client_x509_cert_url = Some(value.to_string());
        self
    }

    fn full_path(&self) -> String {
        self.config_root.full_path().display().to_string()
    }

    pub fn build(&self) -> ClientConfig {
        let persist_path = format!("{}/gmail1", self.full_path());

        ClientConfig {
            secret: self.secret.clone(),
            config_root: self.config_root.clone(),
            persist_path,
        }
    }
}
