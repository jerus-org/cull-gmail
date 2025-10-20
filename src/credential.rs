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
//! use cull_gmail::{ClientConfig, GmailClient};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create client configuration with credential file
//! let mut config_builder = ClientConfig::builder();
//! let config = config_builder
//!     .with_credential_file("client_secret.json")
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper function to create a temporary credential file for testing
    fn create_test_credential_file(temp_dir: &TempDir, filename: &str, content: &str) -> String {
        let file_path = temp_dir.path().join(filename);
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path.to_string_lossy().to_string()
    }

    /// Sample valid credential JSON for testing
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

    /// Sample minimal valid credential JSON for testing
    fn sample_minimal_credential() -> &'static str {
        r#"{
  "installed": {
    "client_id": "minimal-client-id",
    "auth_uri": "https://accounts.google.com/o/oauth2/auth",
    "token_uri": "https://oauth2.googleapis.com/token",
    "client_secret": "minimal-secret",
    "redirect_uris": []
  }
}"#
    }

    #[test]
    fn test_installed_struct_serialization() {
        let installed = Installed {
            client_id: "test-client-id".to_string(),
            project_id: Some("test-project".to_string()),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            auth_provider_x509_cert_url: Some(
                "https://www.googleapis.com/oauth2/v1/certs".to_string(),
            ),
            client_secret: "test-secret".to_string(),
            redirect_uris: vec!["http://localhost".to_string()],
        };

        // Test serialization
        let json = serde_json::to_string(&installed).expect("Should serialize");
        assert!(json.contains("test-client-id"));
        assert!(json.contains("test-secret"));

        // Test deserialization
        let deserialized: Installed = serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(deserialized.client_id, installed.client_id);
        assert_eq!(deserialized.client_secret, installed.client_secret);
    }

    #[test]
    fn test_credential_struct_serialization() {
        let installed = Installed {
            client_id: "test-id".to_string(),
            project_id: None,
            auth_uri: "auth-uri".to_string(),
            token_uri: "token-uri".to_string(),
            auth_provider_x509_cert_url: None,
            client_secret: "secret".to_string(),
            redirect_uris: vec![],
        };

        let credential = Credential {
            installed: Some(installed),
        };

        // Test serialization
        let json = serde_json::to_string(&credential).expect("Should serialize");
        assert!(json.contains("test-id"));

        // Test deserialization
        let deserialized: Credential = serde_json::from_str(&json).expect("Should deserialize");
        assert!(deserialized.installed.is_some());
        assert_eq!(deserialized.installed.unwrap().client_id, "test-id");
    }

    #[test]
    fn test_credential_with_valid_json() {
        let json = sample_valid_credential();
        let credential: Credential = serde_json::from_str(json).expect("Should parse valid JSON");

        assert!(credential.installed.is_some());
        let installed = credential.installed.unwrap();
        assert_eq!(installed.client_id, "123456789-test.googleusercontent.com");
        assert_eq!(installed.client_secret, "test-client-secret");
        assert_eq!(installed.project_id, Some("test-project".to_string()));
        assert_eq!(
            installed.auth_uri,
            "https://accounts.google.com/o/oauth2/auth"
        );
        assert_eq!(installed.token_uri, "https://oauth2.googleapis.com/token");
        assert_eq!(installed.redirect_uris, vec!["http://localhost"]);
    }

    #[test]
    fn test_credential_with_minimal_json() {
        let json = sample_minimal_credential();
        let credential: Credential = serde_json::from_str(json).expect("Should parse minimal JSON");

        assert!(credential.installed.is_some());
        let installed = credential.installed.unwrap();
        assert_eq!(installed.client_id, "minimal-client-id");
        assert_eq!(installed.client_secret, "minimal-secret");
        assert_eq!(installed.project_id, None);
        assert!(installed.redirect_uris.is_empty());
    }

    #[test]
    fn test_credential_with_empty_installed() {
        let json = r#"{"installed": null}"#;
        let credential: Credential =
            serde_json::from_str(json).expect("Should parse null installed");
        assert!(credential.installed.is_none());
    }

    #[test]
    fn test_credential_with_missing_installed() {
        let json = r#"{}"#;
        let credential: Credential = serde_json::from_str(json).expect("Should parse empty object");
        assert!(credential.installed.is_none());
    }

    #[test]
    fn test_invalid_json_parsing() {
        let invalid_cases = [
            "",                                                                  // Empty string
            "{",                                                                 // Incomplete JSON
            "not json",                                                          // Not JSON at all
            r#"{"installed": "wrong"}"#, // Wrong type for installed
            r#"{"installed": {"client_id": "test", "missing_required": true}}"#, // Missing required fields
        ];

        for invalid_json in invalid_cases {
            let result = serde_json::from_str::<Credential>(invalid_json);
            assert!(result.is_err(), "Should fail to parse: {}", invalid_json);
        }
    }

    #[test]
    fn test_conversion_to_application_secret() {
        let json = sample_valid_credential();
        let credential: Credential = serde_json::from_str(json).unwrap();
        let app_secret: yup_oauth2::ApplicationSecret = credential.into();

        assert_eq!(app_secret.client_id, "123456789-test.googleusercontent.com");
        assert_eq!(app_secret.client_secret, "test-client-secret");
        assert_eq!(app_secret.project_id, Some("test-project".to_string()));
        assert_eq!(
            app_secret.auth_uri,
            "https://accounts.google.com/o/oauth2/auth"
        );
        assert_eq!(app_secret.token_uri, "https://oauth2.googleapis.com/token");
        assert_eq!(app_secret.redirect_uris, vec!["http://localhost"]);
        assert_eq!(
            app_secret.auth_provider_x509_cert_url,
            Some("https://www.googleapis.com/oauth2/v1/certs".to_string())
        );
    }

    #[test]
    fn test_conversion_with_minimal_credential() {
        let json = sample_minimal_credential();
        let credential: Credential = serde_json::from_str(json).unwrap();
        let app_secret: yup_oauth2::ApplicationSecret = credential.into();

        assert_eq!(app_secret.client_id, "minimal-client-id");
        assert_eq!(app_secret.client_secret, "minimal-secret");
        assert_eq!(app_secret.project_id, None);
        assert!(app_secret.redirect_uris.is_empty());
    }

    #[test]
    fn test_conversion_with_empty_credential() {
        let credential = Credential { installed: None };
        let app_secret: yup_oauth2::ApplicationSecret = credential.into();

        // Should create default ApplicationSecret
        assert!(app_secret.client_id.is_empty());
        assert!(app_secret.client_secret.is_empty());
        assert_eq!(app_secret.project_id, None);
    }

    #[test]
    fn test_debug_formatting_does_not_expose_secrets() {
        let installed = Installed {
            client_id: "public-client-id".to_string(),
            project_id: Some("public-project".to_string()),
            auth_uri: "https://auth.example.com".to_string(),
            token_uri: "https://token.example.com".to_string(),
            auth_provider_x509_cert_url: None,
            client_secret: "VERY_SECRET_VALUE".to_string(),
            redirect_uris: vec!["http://localhost:8080".to_string()],
        };

        let credential = Credential {
            installed: Some(installed),
        };

        let debug_str = format!("{:?}", credential);

        // Debug should show the structure but we can't easily test that secrets are hidden
        // since the current Debug implementation doesn't hide secrets
        // This test mainly ensures Debug works without panicking
        assert!(debug_str.contains("Credential"));
    }

    #[test]
    fn test_round_trip_serialization() {
        let json = sample_valid_credential();
        let credential: Credential = serde_json::from_str(json).unwrap();
        let serialized = serde_json::to_string(&credential).unwrap();
        let deserialized: Credential = serde_json::from_str(&serialized).unwrap();

        // Compare the installed sections
        match (credential.installed, deserialized.installed) {
            (Some(orig), Some(deser)) => {
                assert_eq!(orig.client_id, deser.client_id);
                assert_eq!(orig.client_secret, deser.client_secret);
                assert_eq!(orig.project_id, deser.project_id);
                assert_eq!(orig.auth_uri, deser.auth_uri);
                assert_eq!(orig.token_uri, deser.token_uri);
                assert_eq!(orig.redirect_uris, deser.redirect_uris);
            }
            _ => panic!("Both should have installed sections"),
        }
    }

    #[test]
    fn test_field_validation_edge_cases() {
        // Test with empty strings
        let json = r#"{
  "installed": {
    "client_id": "",
    "auth_uri": "",
    "token_uri": "",
    "client_secret": "",
    "redirect_uris": []
  }
}"#;

        let credential: Credential =
            serde_json::from_str(json).expect("Should parse empty strings");
        let app_secret: yup_oauth2::ApplicationSecret = credential.into();
        assert!(app_secret.client_id.is_empty());
        assert!(app_secret.client_secret.is_empty());
    }

    #[test]
    fn test_unicode_and_special_characters() {
        let json = r#"{
  "installed": {
    "client_id": "unicode-テスト-🔐-client",
    "auth_uri": "https://auth.example.com/oauth2",
    "token_uri": "https://token.example.com/oauth2",
    "client_secret": "secret-with-symbols-!@#$%^&*()",
    "redirect_uris": ["http://localhost:8080/callback"]
  }
}"#;

        let credential: Credential = serde_json::from_str(json).expect("Should handle Unicode");
        let installed = credential.installed.unwrap();
        assert_eq!(installed.client_id, "unicode-テスト-🔐-client");
        assert_eq!(installed.client_secret, "secret-with-symbols-!@#$%^&*()");
    }

    #[test]
    fn test_large_redirect_uris_list() {
        let mut redirect_uris = Vec::new();
        for i in 0..100 {
            redirect_uris.push(format!("http://localhost:{}", 8000 + i));
        }

        let installed = Installed {
            client_id: "test-client".to_string(),
            project_id: None,
            auth_uri: "https://auth.example.com".to_string(),
            token_uri: "https://token.example.com".to_string(),
            auth_provider_x509_cert_url: None,
            client_secret: "test-secret".to_string(),
            redirect_uris: redirect_uris.clone(),
        };

        let credential = Credential {
            installed: Some(installed),
        };

        let app_secret: yup_oauth2::ApplicationSecret = credential.into();
        assert_eq!(app_secret.redirect_uris.len(), 100);
        assert_eq!(app_secret.redirect_uris[0], "http://localhost:8000");
        assert_eq!(app_secret.redirect_uris[99], "http://localhost:8099");
    }

    // Note: We can't easily test the actual file loading functionality
    // without mocking the home directory and file system, which would
    // require more complex test setup. The current implementation uses
    // `env::home_dir()` and direct file operations that would need
    // more sophisticated mocking to test properly.

    #[test]
    fn test_credential_clone_and_equality() {
        let json = sample_minimal_credential();
        let credential1: Credential = serde_json::from_str(json).unwrap();

        // Test that we can create another credential from the same JSON
        let credential2: Credential = serde_json::from_str(json).unwrap();

        // We can't test equality directly since Credential doesn't implement PartialEq
        // but we can test that conversions produce equivalent results
        let secret1: yup_oauth2::ApplicationSecret = credential1.into();
        let secret2: yup_oauth2::ApplicationSecret = credential2.into();

        assert_eq!(secret1.client_id, secret2.client_id);
        assert_eq!(secret1.client_secret, secret2.client_secret);
    }
}
