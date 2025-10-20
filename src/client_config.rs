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

#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;
    use std::env;
    use tempfile::TempDir;
    use std::fs;
    use crate::test_utils::get_test_logger;

    /// Helper function to create a temporary credential file for testing
    fn create_test_credential_file(temp_dir: &TempDir, filename: &str, content: &str) -> String {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path.to_string_lossy().to_string()
    }

    /// Sample valid OAuth2 credential JSON for testing
    fn sample_valid_credential() -> &'static str {
        r#"{
  "installed": {
    "client_id": "123456789-test.googleusercontent.com",
    "project_id": "test-project",
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://oauth2.googleapis.com/token",
    "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
    "client_secret": "test-client-secret",
    "redirect_uris": ["http://localhost"]
  }
}"#
    }

    #[test]
    fn test_config_builder_defaults() {
        let builder = ConfigBuilder::default();
        
        assert_eq!(builder.secret.auth_uri, "https://accounts.google.com/o/oauth2/auth");
        assert_eq!(builder.secret.token_uri, "https://oauth2.googleapis.com/token");
        assert!(builder.secret.client_id.is_empty());
        assert!(builder.secret.client_secret.is_empty());
    }

    #[test]
    fn test_builder_pattern_direct_oauth2() {
        let config = ClientConfig::builder()
            .with_client_id("test-client-id")
            .with_client_secret("test-client-secret")
            .with_auth_uri("https://auth.example.com")
            .with_token_uri("https://token.example.com")
            .add_redirect_uri("http://localhost:8080")
            .add_redirect_uri("http://localhost:3000")
            .with_project_id("test-project")
            .with_client_email("test@example.com")
            .with_auth_provider_x509_cert_url("https://certs.example.com")
            .with_client_x509_cert_url("https://client-cert.example.com")
            .build();

        assert_eq!(config.secret().client_id, "test-client-id");
        assert_eq!(config.secret().client_secret, "test-client-secret");
        assert_eq!(config.secret().auth_uri, "https://auth.example.com");
        assert_eq!(config.secret().token_uri, "https://token.example.com");
        assert_eq!(config.secret().redirect_uris, vec!["http://localhost:8080", "http://localhost:3000"]);
        assert_eq!(config.secret().project_id, Some("test-project".to_string()));
        assert_eq!(config.secret().client_email, Some("test@example.com".to_string()));
        assert_eq!(config.secret().auth_provider_x509_cert_url, Some("https://certs.example.com".to_string()));
        assert_eq!(config.secret().client_x509_cert_url, Some("https://client-cert.example.com".to_string()));
        assert!(config.persist_path().contains("gmail1"));
    }

    #[test]
    fn test_builder_with_config_path() {
        let config = ClientConfig::builder()
            .with_client_id("test-id")
            .with_config_path(".test-config")
            .build();

        let full_path = config.full_path();
        assert_eq!(full_path, ".test-config");
        assert!(config.persist_path().contains(".test-config/gmail1"));
    }

    #[test]
    fn test_builder_with_config_base_home() {
        let config = ClientConfig::builder()
            .with_client_id("test-id")
            .with_config_base(&config_root::RootBase::Home)
            .with_config_path(".test-config")
            .build();

        let expected_path = env::home_dir()
            .unwrap_or_default()
            .join(".test-config")
            .display()
            .to_string();
            
        assert_eq!(config.full_path(), expected_path);
    }

    #[test]
    fn test_builder_with_config_base_root() {
        let config = ClientConfig::builder()
            .with_client_id("test-id")
            .with_config_base(&config_root::RootBase::Root)
            .with_config_path("etc/test-config")
            .build();

        assert_eq!(config.full_path(), "/etc/test-config");
    }

    #[test]
    fn test_config_from_direct_oauth2_params() {
        let app_config = Config::builder()
            .set_default("client_id", "direct-client-id").unwrap()
            .set_default("client_secret", "direct-client-secret").unwrap()
            .set_default("token_uri", "https://token.direct.com").unwrap()
            .set_default("auth_uri", "https://auth.direct.com").unwrap()
            .set_default("config_root", "h:.test-direct").unwrap()
            .build()
            .unwrap();

        let config = ClientConfig::new_from_configuration(app_config).unwrap();
        
        assert_eq!(config.secret().client_id, "direct-client-id");
        assert_eq!(config.secret().client_secret, "direct-client-secret");
        assert_eq!(config.secret().token_uri, "https://token.direct.com");
        assert_eq!(config.secret().auth_uri, "https://auth.direct.com");
        assert_eq!(config.secret().project_id, None);
        assert!(config.secret().redirect_uris.is_empty());
    }

    #[test]
    fn test_config_from_credential_file() {
        get_test_logger();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let _cred_file = create_test_credential_file(&temp_dir, "test_creds.json", sample_valid_credential());
        
        let config_root = format!("c:{}", temp_dir.path().display());
        let app_config = Config::builder()
            .set_default("credential_file", "test_creds.json").unwrap()
            .set_default("config_root", config_root.as_str()).unwrap()
            .build()
            .unwrap();

        let config = ClientConfig::new_from_configuration(app_config).unwrap();
        
        assert_eq!(config.secret().client_id, "123456789-test.googleusercontent.com");
        assert_eq!(config.secret().client_secret, "test-client-secret");
        assert_eq!(config.secret().project_id, Some("test-project".to_string()));
        assert_eq!(config.secret().redirect_uris, vec!["http://localhost"]);
    }

    #[test]
    fn test_config_missing_required_params() {
        // Test with missing config_root
        let app_config = Config::builder()
            .set_default("client_id", "test-id").unwrap()
            .build()
            .unwrap();

        let result = ClientConfig::new_from_configuration(app_config);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_incomplete_oauth2_params() {
        // Test with some but not all OAuth2 parameters
        let app_config = Config::builder()
            .set_default("client_id", "test-id").unwrap()
            .set_default("client_secret", "test-secret").unwrap()
            // Missing token_uri and auth_uri
            .set_default("config_root", "h:.test").unwrap()
            .build()
            .unwrap();

        // Should fall back to credential_file approach, which should fail
        let result = ClientConfig::new_from_configuration(app_config);
        assert!(result.is_err());
    }

    #[test] 
    #[should_panic(expected = "could not read path")]
    fn test_config_invalid_credential_file() {
        let app_config = Config::builder()
            .set_default("credential_file", "nonexistent.json").unwrap()
            .set_default("config_root", "h:.test").unwrap()
            .build()
            .unwrap();

        // This should panic with "could not read path" since the code uses .expect()
        let _result = ClientConfig::new_from_configuration(app_config);
    }

    #[test]
    #[should_panic(expected = "could not convert to struct")]
    fn test_config_malformed_credential_file() {
        get_test_logger();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let _cred_file = create_test_credential_file(&temp_dir, "malformed.json", "{ invalid json");
        
        let config_root = format!("c:{}", temp_dir.path().display());
        let app_config = Config::builder()
            .set_default("credential_file", "malformed.json").unwrap()
            .set_default("config_root", config_root.as_str()).unwrap()
            .build()
            .unwrap();

        // This should panic with "could not convert to struct" since the code uses .expect()
        let _result = ClientConfig::new_from_configuration(app_config);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_config_credential_file_wrong_structure() {
        get_test_logger();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let wrong_structure = r#"{"wrong": "structure"}"#;
        let _cred_file = create_test_credential_file(&temp_dir, "wrong.json", wrong_structure);
        
        let config_root = format!("c:{}", temp_dir.path().display());
        let app_config = Config::builder()
            .set_default("credential_file", "wrong.json").unwrap()
            .set_default("config_root", config_root.as_str()).unwrap()
            .build()
            .unwrap();

        // This should panic with unwrap on None since console.installed is None
        let _result = ClientConfig::new_from_configuration(app_config);
    }

    #[test]
    fn test_persist_path_generation() {
        let config = ClientConfig::builder()
            .with_client_id("test")
            .with_config_path("/custom/path")
            .build();

        assert_eq!(config.persist_path(), "/custom/path/gmail1");
    }

    #[test]
    fn test_config_accessor_methods() {
        let config = ClientConfig::builder()
            .with_client_id("accessor-test-id")
            .with_client_secret("accessor-test-secret")
            .with_config_path("/test/path")
            .build();

        // Test secret() accessor
        let secret = config.secret();
        assert_eq!(secret.client_id, "accessor-test-id");
        assert_eq!(secret.client_secret, "accessor-test-secret");

        // Test persist_path() accessor
        assert_eq!(config.persist_path(), "/test/path/gmail1");

        // Test full_path() accessor
        assert_eq!(config.full_path(), "/test/path");

        // Test config_root() accessor
        let config_root = config.config_root();
        assert_eq!(config_root.full_path().display().to_string(), "/test/path");
    }

    #[test]
    fn test_builder_method_chaining() {
        // Test that all builder methods return &mut Self for chaining
        let config = ClientConfig::builder()
            .with_client_id("chain-test")
            .with_client_secret("chain-secret")
            .with_auth_uri("https://auth.chain.com")
            .with_token_uri("https://token.chain.com")
            .add_redirect_uri("http://redirect1.com")
            .add_redirect_uri("http://redirect2.com")
            .with_project_id("chain-project")
            .with_client_email("chain@test.com")
            .with_auth_provider_x509_cert_url("https://cert1.com")
            .with_client_x509_cert_url("https://cert2.com")
            .with_config_base(&config_root::RootBase::Home)
            .with_config_path(".chain-test")
            .build();

        assert_eq!(config.secret().client_id, "chain-test");
        assert_eq!(config.secret().redirect_uris.len(), 2);
    }

    #[test]
    fn test_configuration_priority() {
        // Test that direct OAuth2 params take priority over credential file
        get_test_logger();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let _cred_file = create_test_credential_file(&temp_dir, "priority.json", sample_valid_credential());
        
        let config_root = format!("c:{}", temp_dir.path().display());
        let app_config = Config::builder()
            // Direct OAuth2 params (should take priority)
            .set_default("client_id", "priority-client-id").unwrap()
            .set_default("client_secret", "priority-client-secret").unwrap()
            .set_default("token_uri", "https://priority.token.com").unwrap()
            .set_default("auth_uri", "https://priority.auth.com").unwrap()
            // Credential file (should be ignored)
            .set_default("credential_file", "priority.json").unwrap()
            .set_default("config_root", config_root.as_str()).unwrap()
            .build()
            .unwrap();

        let config = ClientConfig::new_from_configuration(app_config).unwrap();
        
        // Should use direct params, not file contents
        assert_eq!(config.secret().client_id, "priority-client-id");
        assert_eq!(config.secret().client_secret, "priority-client-secret");
        assert_eq!(config.secret().token_uri, "https://priority.token.com");
        assert_ne!(config.secret().client_id, "123456789-test.googleusercontent.com"); // From file
    }

    #[test] 
    fn test_empty_redirect_uris() {
        let config = ClientConfig::builder()
            .with_client_id("test-id")
            .build();

        assert!(config.secret().redirect_uris.is_empty());
    }

    #[test]
    fn test_multiple_redirect_uris() {
        let config = ClientConfig::builder()
            .with_client_id("test-id")
            .add_redirect_uri("http://localhost:8080")
            .add_redirect_uri("http://localhost:3000")
            .add_redirect_uri("https://example.com/callback")
            .build();

        assert_eq!(config.secret().redirect_uris.len(), 3);
        assert!(config.secret().redirect_uris.contains(&"http://localhost:8080".to_string()));
        assert!(config.secret().redirect_uris.contains(&"http://localhost:3000".to_string()));
        assert!(config.secret().redirect_uris.contains(&"https://example.com/callback".to_string()));
    }

    #[test]
    fn test_optional_fields() {
        let config = ClientConfig::builder()
            .with_client_id("optional-test")
            .build();

        assert_eq!(config.secret().project_id, None);
        assert_eq!(config.secret().client_email, None);
        assert_eq!(config.secret().auth_provider_x509_cert_url, None);
        assert_eq!(config.secret().client_x509_cert_url, None);
    }

    #[test]
    fn test_unicode_in_configuration() {
        let config = ClientConfig::builder()
            .with_client_id("unicode-客戶端-🔐-test")
            .with_client_secret("secret-with-unicode-密碼")
            .with_project_id("project-項目-id")
            .with_config_path(".unicode-配置")
            .build();

        assert_eq!(config.secret().client_id, "unicode-客戶端-🔐-test");
        assert_eq!(config.secret().client_secret, "secret-with-unicode-密碼");
        assert_eq!(config.secret().project_id, Some("project-項目-id".to_string()));
        assert!(config.full_path().contains(".unicode-配置"));
    }
}
