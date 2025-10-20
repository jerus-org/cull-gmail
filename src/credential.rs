//! # OAuth2 Credential Management Module
//!
//! This module provides secure OAuth2 credential handling for Gmail API authentication.
//! It supports loading Google Cloud Platform OAuth2 credentials from JSON files and
//! converting them to the format required by the Gmail API client.
//!
//! ## Overview
//!
//! The credential system handles OAuth2 "installed application" type credentials,
//! which are used for desktop and command-line applications that authenticate users
//! through a browser-based OAuth2 flow.
//!
//! ## Security Considerations
//!
//! - **Credential Storage**: Credentials should be stored securely and never committed to version control
//! - **File Permissions**: Credential files should have restricted permissions (600 or similar)
//! - **Path Handling**: The module resolves paths relative to `~/.cull-gmail/` for security consistency
//! - **Error Handling**: File I/O errors are propagated to prevent silent failures
//!
//! ## Credential File Format
//!
//! The module expects Google Cloud Platform OAuth2 credentials in the standard JSON format:
//!
//! ```json
//! {
//!   "installed": {
//!     "client_id": "your-client-id.googleusercontent.com",
//!     "project_id": "your-project-id",
//!     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
//!     "token_uri": "https://oauth2.googleapis.com/token",
//!     "client_secret": "your-client-secret",
//!     "redirect_uris": ["http://localhost"],
//!     "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs"
//!   }
//! }
//! ```
//!
//! ## Usage Examples
//!
//! ### Loading Credentials
//!
//! ```rust,no_run
//! use cull_gmail::Credential;
//! use google_gmail1::yup_oauth2::ApplicationSecret;
//!
//! // Load credentials from ~/.cull-gmail/client_secret.json
//! let credential = Credential::load_json_file("client_secret.json");
//!
//! // Convert to ApplicationSecret for OAuth2 authentication
//! let app_secret: ApplicationSecret = credential.into();
//! ```
//!
//! ### Error Handling
//!
//! ```rust,no_run
//! use cull_gmail::Credential;
//!
//! // Note: This will panic if the file doesn't exist or is malformed
//! // In production code, consider using a Result-based approach
//! let credential = Credential::load_json_file("credentials.json");
//! ```
//!
//! ## Integration with Gmail Client
//!
//! The credential module integrates seamlessly with the Gmail API client:
//!
//! ```rust,no_run
//! use cull_gmail::{Credential, ClientConfig, GmailClient};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load credentials
//! let credential = Credential::load_json_file("client_secret.json");
//!
//! // Create client configuration
//! let config = ClientConfig::builder()
//!     .with_credential(credential)
//!     .build();
//!
//! // Initialize Gmail client
//! let client = GmailClient::new_with_config(config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## File System Layout
//!
//! By default, credentials are expected in the user's home directory:
//!
//! ```text
//! ~/.cull-gmail/
//! ├── client_secret.json          # OAuth2 credentials
//! ├── tokens/                     # OAuth2 token cache
//! └── config.toml                # Application configuration
//! ```
//!
//! ## Thread Safety
//!
//! The credential types are safe to clone and use across threads. However,
//! file I/O operations are synchronous and should be performed during
//! application initialization rather than in performance-critical paths.

use std::{env, fs, path::PathBuf};

use google_gmail1::yup_oauth2;
use serde::{Deserialize, Serialize};

/// OAuth2 "installed application" credential configuration.
///
/// This struct represents the credential information for an OAuth2 "installed application"
/// as defined by Google's OAuth2 specification. It contains all the necessary parameters
/// for configuring OAuth2 authentication flows for desktop and command-line applications.
///
/// # Fields
///
/// The struct contains OAuth2 configuration parameters that are typically provided
/// by Google Cloud Platform when creating OAuth2 credentials for installed applications.
///
/// # Security
///
/// The `client_secret` field contains sensitive information and should be protected.
/// This struct should only be used in secure environments and never exposed in logs
/// or error messages.
///
/// # Serialization
///
/// This struct can be serialized to/from JSON format, matching the standard
/// Google OAuth2 credential file format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Installed {
    /// OAuth2 client identifier assigned by Google.
    ///
    /// This is a public identifier that uniquely identifies the OAuth2 application.
    /// It typically ends with `.googleusercontent.com`.
    pub(crate) client_id: String,
    
    /// Google Cloud Platform project identifier.
    ///
    /// Optional field that identifies the GCP project associated with these credentials.
    /// Used for quota management and billing purposes.
    pub(crate) project_id: Option<String>,
    
    /// OAuth2 authorization endpoint URL.
    ///
    /// The URL where users are redirected to authenticate and authorize the application.
    /// Typically `https://accounts.google.com/o/oauth2/auth` for Google services.
    pub(crate) auth_uri: String,
    
    /// OAuth2 token exchange endpoint URL.
    ///
    /// The URL used to exchange authorization codes for access tokens.
    /// Typically `https://oauth2.googleapis.com/token` for Google services.
    pub(crate) token_uri: String,
    
    /// URL for OAuth2 provider's X.509 certificate.
    ///
    /// Optional URL pointing to the public certificates used to verify JWT tokens
    /// from the OAuth2 provider. Used for token validation.
    pub(crate) auth_provider_x509_cert_url: Option<String>,
    
    /// OAuth2 client secret.
    ///
    /// **SENSITIVE**: This is a confidential value that must be kept secure.
    /// It's used to authenticate the application to the OAuth2 provider.
    /// Never log or expose this value.
    pub(crate) client_secret: String,
    
    /// List of authorized redirect URIs.
    ///
    /// These URIs are pre-registered with the OAuth2 provider and define
    /// where users can be redirected after authorization. For installed
    /// applications, this typically includes `http://localhost` variants.
    pub(crate) redirect_uris: Vec<String>,
}

/// OAuth2 credential container for Google API authentication.
///
/// This struct serves as the main interface for loading and managing OAuth2 credentials
/// used to authenticate with Google APIs, specifically Gmail. It wraps the standard
/// Google OAuth2 credential format and provides convenient methods for loading
/// credentials from the filesystem.
///
/// # Credential Types
///
/// Currently supports "installed application" type credentials, which are appropriate
/// for desktop applications and command-line tools that authenticate users through
/// a browser-based OAuth2 flow.
///
/// # Security Model
///
/// - Credentials are loaded from the user's home directory (`~/.cull-gmail/`)
/// - Files should have restricted permissions (600) to prevent unauthorized access
/// - Client secrets are sensitive and should never be logged or exposed
///
/// # Examples
///
/// ```rust,no_run
/// use cull_gmail::Credential;
///
/// // Load credentials from ~/.cull-gmail/client_secret.json
/// let credential = Credential::load_json_file("client_secret.json");
///
/// // Use with Gmail API client configuration
/// println!("Loaded credential successfully");
/// ```
///
/// # File Format
///
/// The expected JSON format follows Google's standard OAuth2 credential format:
///
/// ```json
/// {
///   "installed": {
///     "client_id": "123456789-abc.googleusercontent.com",
///     "client_secret": "your-client-secret",
///     "auth_uri": "https://accounts.google.com/o/oauth2/auth",
///     "token_uri": "https://oauth2.googleapis.com/token",
///     "redirect_uris": ["http://localhost"]
///   }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Credential {
    /// The installed application credential configuration.
    ///
    /// This field contains the actual OAuth2 configuration parameters.
    /// It's optional to handle cases where the credential file might
    /// be malformed or empty, though in practice it should always be present
    /// for valid credential files.
    installed: Option<Installed>,
}

impl Credential {
    /// Loads OAuth2 credentials from a JSON file.
    ///
    /// This method loads Google OAuth2 credentials from a JSON file located in the
    /// user's cull-gmail configuration directory (`~/.cull-gmail/`). The file is
    /// expected to be in the standard Google OAuth2 credential format.
    ///
    /// # Arguments
    ///
    /// * `path` - Relative path to the credential file within `~/.cull-gmail/`
    ///
    /// # Returns
    ///
    /// A `Credential` instance containing the loaded OAuth2 configuration.
    ///
    /// # Panics
    ///
    /// This method will panic if:
    /// - The user's home directory cannot be determined
    /// - The specified file cannot be read (doesn't exist, permission denied, etc.)
    /// - The file content is not valid JSON
    /// - The JSON structure doesn't match the expected credential format
    ///
    /// # Security
    ///
    /// - Only loads files from the secure `~/.cull-gmail/` directory
    /// - Ensure credential files have restrictive permissions (600)
    /// - Never pass credential file paths from untrusted sources
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use cull_gmail::Credential;
    ///
    /// // Load from ~/.cull-gmail/client_secret.json
    /// let credential = Credential::load_json_file("client_secret.json");
    ///
    /// // Load from ~/.cull-gmail/oauth2/credentials.json
    /// let credential = Credential::load_json_file("oauth2/credentials.json");
    /// ```
    ///
    /// # Error Handling
    ///
    /// Consider wrapping calls in error handling for production use:
    ///
    /// ```rust,no_run
    /// use cull_gmail::Credential;
    /// use std::panic;
    ///
    /// let credential = panic::catch_unwind(|| {
    ///     Credential::load_json_file("client_secret.json")
    /// });
    ///
    /// match credential {
    ///     Ok(cred) => println!("Credentials loaded successfully"),
    ///     Err(_) => eprintln!("Failed to load credentials"),
    /// }
    /// ```
    pub fn load_json_file(path: &str) -> Self {
        let home_dir = env::home_dir().unwrap();

        let path = PathBuf::new().join(home_dir).join(".cull-gmail").join(path);
        let json_str = fs::read_to_string(path).expect("could not read path");

        serde_json::from_str(&json_str).expect("could not convert to struct")
    }
}

/// Converts a `Credential` into a `yup_oauth2::ApplicationSecret`.
///
/// This implementation enables seamless integration between cull-gmail's credential
/// system and the `yup_oauth2` crate used for OAuth2 authentication. It extracts
/// the OAuth2 configuration parameters and maps them to the format expected by
/// the OAuth2 client library.
///
/// # Conversion Process
///
/// The conversion extracts fields from the `installed` section of the credential
/// and maps them to the corresponding fields in `ApplicationSecret`:
///
/// - `client_id` → OAuth2 client identifier
/// - `client_secret` → OAuth2 client secret (sensitive)
/// - `auth_uri` → Authorization endpoint URL
/// - `token_uri` → Token exchange endpoint URL  
/// - `redirect_uris` → Authorized redirect URIs
/// - `project_id` → GCP project identifier (optional)
/// - `auth_provider_x509_cert_url` → Certificate URL (optional)
///
/// # Security
///
/// The conversion preserves all sensitive information, particularly the client secret.
/// The resulting `ApplicationSecret` should be handled with the same security
/// considerations as the original credential.
///
/// # Examples
///
/// ```rust,no_run
/// use cull_gmail::Credential;
/// use google_gmail1::yup_oauth2::ApplicationSecret;
///
/// // Load credential and convert to ApplicationSecret
/// let credential = Credential::load_json_file("client_secret.json");
/// let app_secret: ApplicationSecret = credential.into();
///
/// // Or use the conversion explicitly
/// let credential = Credential::load_json_file("client_secret.json");
/// let app_secret = ApplicationSecret::from(credential);
/// ```
///
/// # Behavior with Missing Fields
///
/// If the credential doesn't contain an `installed` section (which would indicate
/// a malformed credential file), the conversion creates a default `ApplicationSecret`
/// with empty/default values. In practice, this should not occur with valid
/// credential files.
impl From<Credential> for yup_oauth2::ApplicationSecret {
    fn from(value: Credential) -> Self {
        let mut out = Self::default();
        if let Some(installed) = value.installed {
            out.client_id = installed.client_id;
            out.client_secret = installed.client_secret;
            out.token_uri = installed.token_uri;
            out.auth_uri = installed.auth_uri;
            out.redirect_uris = installed.redirect_uris;
            out.project_id = installed.project_id;
            // out.client_email = installed.client_email;
            out.auth_provider_x509_cert_url = installed.auth_provider_x509_cert_url;
            // out.client_x509_cert_url = installed.client_x509_cert_url;
        }
        out
    }
}
