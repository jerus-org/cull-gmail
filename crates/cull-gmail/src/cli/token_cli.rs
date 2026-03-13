//! # Token Management CLI Module
//!
//! This module provides CLI functionality for exporting and importing OAuth2 tokens
//! to support running the application in ephemeral environments like containers or CI/CD pipelines.
//!
//! ## Overview
//!
//! The token management system allows users to:
//!
//! - **Export tokens**: Extract current OAuth2 tokens to a compressed base64 string
//! - **Import tokens**: Recreate token files from environment variables
//! - **Ephemeral workflows**: Run in clean environments by restoring tokens from env vars
//!
//! ## Use Cases
//!
//! ### Container Deployments
//! ```bash
//! # Export tokens from development environment
//! cull-gmail token export
//!
//! # Set environment variable in container
//! docker run -e CULL_GMAIL_TOKEN_CACHE="<exported-string>" my-app
//! ```
//!
//! ### CI/CD Pipelines
//! ```bash
//! # Store tokens as secret in CI system
//! cull-gmail token export > token.secret
//!
//! # Use in pipeline
//! export CULL_GMAIL_TOKEN_CACHE=$(cat token.secret)
//! cull-gmail messages list --query "older_than:30d"
//! ```
//!
//! ### Periodic Jobs
//! ```bash
//! # One-time setup: export tokens
//! TOKENS=$(cull-gmail token export)
//!
//! # Recurring job: restore and use
//! export CULL_GMAIL_TOKEN_CACHE="$TOKENS"
//! cull-gmail rules run
//! ```
//!
//! ## Security Considerations
//!
//! - **Token Sensitivity**: Exported tokens contain OAuth2 refresh tokens - treat as secrets
//! - **Environment Variables**: Use secure secret management for token storage
//! - **Expiration**: Tokens may expire and require re-authentication
//! - **Scope Limitations**: Exported tokens maintain original OAuth2 scope restrictions
//!
//! ## Token Format
//!
//! Exported tokens are compressed JSON structures containing:
//! - OAuth2 access tokens
//! - Refresh tokens  
//! - Token metadata and expiration
//! - Encoded as base64 for environment variable compatibility

use crate::{ClientConfig, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD as Base64Engine};
use clap::Subcommand;
use cull_gmail::Error;
use std::fs;
use std::path::Path;

/// Token management operations for ephemeral environments.
///
/// This CLI subcommand provides functionality to export OAuth2 tokens to compressed
/// strings suitable for environment variables, and import them in clean environments
/// to avoid re-authentication flows.
///
/// ## Subcommands
///
/// - **export**: Export current tokens to stdout as base64-encoded string
/// - **import**: Import tokens from environment variable (typically automatic)
///
/// ## Usage Examples
///
/// ### Export Tokens
/// ```bash
/// # Export to stdout
/// cull-gmail token export
///
/// # Export to file
/// cull-gmail token export > tokens.env
///
/// # Export to environment variable
/// export MY_TOKENS=$(cull-gmail token export)
/// ```
///
/// ### Import Usage
/// ```bash
/// # Set environment variable
/// export CULL_GMAIL_TOKEN_CACHE="<base64-string>"
///
/// # Run normally - tokens will be restored automatically
/// cull-gmail labels
/// ```
#[derive(clap::Parser, Debug)]
pub struct TokenCli {
    #[command(subcommand)]
    command: TokenCommand,
}

/// Available token management operations.
///
/// Each operation handles different aspects of token lifecycle management
/// for ephemeral environment support.
#[derive(Subcommand, Debug)]
pub enum TokenCommand {
    /// Export current OAuth2 tokens to a compressed string.
    ///
    /// This command reads the current token cache and outputs a base64-encoded,
    /// compressed representation suitable for storage in environment variables
    /// or CI/CD secret systems.
    ///
    /// ## Output Format
    ///
    /// The output is a single line containing a base64-encoded string that represents
    /// the compressed JSON structure of all OAuth2 tokens and metadata.
    ///
    /// ## Examples
    ///
    /// ```bash
    /// # Basic export
    /// cull-gmail token export
    ///
    /// # Store in environment variable
    /// export TOKENS=$(cull-gmail token export)
    ///
    /// # Save to file
    /// cull-gmail token export > token.secret
    /// ```
    Export,

    /// Import OAuth2 tokens from environment variable.
    ///
    /// This command is typically not called directly, as token import happens
    /// automatically during client initialization when the CULL_GMAIL_TOKEN_CACHE
    /// environment variable is present.
    ///
    /// ## Manual Import
    ///
    /// ```bash
    /// # Set the environment variable
    /// export CULL_GMAIL_TOKEN_CACHE="<base64-string>"
    ///
    /// # Import explicitly (usually automatic)
    /// cull-gmail token import
    /// ```
    Import,
}

impl TokenCli {
    /// Execute the token management command.
    ///
    /// This method dispatches to the appropriate token operation based on the
    /// selected subcommand and handles the complete workflow for token export
    /// or import operations.
    ///
    /// # Arguments
    ///
    /// * `client_config` - Client configuration containing token storage paths
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` indicating success or failure of the token operation.
    ///
    /// # Errors
    ///
    /// - File I/O errors when reading or writing token files
    /// - Serialization errors when processing token data
    /// - Environment variable errors during import operations
    pub async fn run(&self, client_config: &ClientConfig) -> Result<()> {
        match &self.command {
            TokenCommand::Export => export_tokens(client_config).await,
            TokenCommand::Import => import_tokens(client_config).await,
        }
    }
}

/// Export OAuth2 tokens to a compressed base64 string.
///
/// This function reads the token cache directory, compresses all token files,
/// and outputs a base64-encoded string suitable for environment variable storage.
///
/// # Arguments
///
/// * `config` - Client configuration containing token persistence path
///
/// # Returns
///
/// Returns `Result<()>` with the base64 string printed to stdout, or an error
/// if token files cannot be read or processed.
///
/// # Process Flow
///
/// 1. **Read Token Directory**: Scan the OAuth2 token persistence directory
/// 2. **Collect Token Files**: Read all token-related files and metadata
/// 3. **Compress Data**: Use gzip compression on the JSON structure
/// 4. **Encode**: Convert to base64 for environment variable compatibility
/// 5. **Output**: Print the resulting string to stdout
///
/// # Errors
///
/// - `Error::TokenNotFound` - No token cache directory or files found
/// - I/O errors reading token files
/// - Serialization errors processing token data
async fn export_tokens(config: &ClientConfig) -> Result<()> {
    let token_path = Path::new(config.persist_path());
    let mut token_data = std::collections::HashMap::new();

    if token_path.is_file() {
        // OAuth2 token is stored as a single file
        let filename = token_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::FileIo("Invalid token filename".to_string()))?;

        let content = fs::read_to_string(token_path)
            .map_err(|e| Error::FileIo(format!("Failed to read token file: {e}")))?;

        token_data.insert(filename.to_string(), content);
    } else if token_path.is_dir() {
        // Token directory with multiple files (legacy support)
        for entry in fs::read_dir(token_path).map_err(|e| Error::FileIo(e.to_string()))? {
            let entry = entry.map_err(|e| Error::FileIo(e.to_string()))?;
            let path = entry.path();

            if path.is_file() {
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| Error::FileIo("Invalid filename in token cache".to_string()))?;

                let content = fs::read_to_string(&path).map_err(|e| {
                    Error::FileIo(format!("Failed to read token file {filename}: {e}"))
                })?;

                token_data.insert(filename.to_string(), content);
            }
        }
    } else {
        return Err(Error::TokenNotFound(format!(
            "Token cache not found: {}",
            token_path.display()
        )));
    }

    if token_data.is_empty() {
        return Err(Error::TokenNotFound(
            "No token data found in cache".to_string(),
        ));
    }

    // Serialize to JSON
    let json_data = serde_json::to_string(&token_data)
        .map_err(|e| Error::SerializationError(format!("Failed to serialize token data: {e}")))?;

    // Compress using flate2
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(json_data.as_bytes())
        .map_err(|e| Error::SerializationError(format!("Failed to compress token data: {e}")))?;
    let compressed_data = encoder
        .finish()
        .map_err(|e| Error::SerializationError(format!("Failed to finalize compression: {e}")))?;

    // Encode to base64
    let encoded = Base64Engine.encode(&compressed_data);

    // Output to stdout
    println!("{encoded}");

    Ok(())
}

/// Import OAuth2 tokens from environment variable.
///
/// This function reads the CULL_GMAIL_TOKEN_CACHE environment variable,
/// decompresses the token data, and recreates the token cache files.
///
/// # Arguments
///
/// * `config` - Client configuration containing token persistence path
///
/// # Returns
///
/// Returns `Result<()>` indicating successful token restoration or an error
/// if the environment variable is missing or token data cannot be processed.
///
/// # Process Flow
///
/// 1. **Read Environment**: Get CULL_GMAIL_TOKEN_CACHE environment variable
/// 2. **Decode**: Base64 decode the token string
/// 3. **Decompress**: Gunzip the token data
/// 4. **Parse**: Deserialize JSON token structure
/// 5. **Recreate Files**: Write token files to cache directory
/// 6. **Set Permissions**: Ensure appropriate file permissions for security
///
/// # Errors
///
/// - `Error::TokenNotFound` - Environment variable not set
/// - Decoding/decompression errors for malformed token data
/// - I/O errors creating token files
pub async fn import_tokens(config: &ClientConfig) -> Result<()> {
    let token_env = std::env::var("CULL_GMAIL_TOKEN_CACHE").map_err(|_| {
        Error::TokenNotFound("CULL_GMAIL_TOKEN_CACHE environment variable not set".to_string())
    })?;

    restore_tokens_from_string(&token_env, config.persist_path())?;

    log::info!("Tokens successfully imported from environment variable");
    Ok(())
}

/// Restore token files from a compressed base64 string.
///
/// This internal function handles the complete token restoration process,
/// including decoding, decompression, and file recreation.
///
/// # Arguments
///
/// * `token_string` - Base64-encoded compressed token data
/// * `persist_path` - Directory path where token files should be created
///
/// # Returns
///
/// Returns `Result<()>` indicating successful restoration or processing errors.
///
/// # File Permissions
///
/// Created token files are set to 600 (owner read/write only) for security.
pub fn restore_tokens_from_string(token_string: &str, persist_path: &str) -> Result<()> {
    // Decode from base64
    let compressed_data = Base64Engine.decode(token_string.trim()).map_err(|e| {
        Error::SerializationError(format!("Failed to decode base64 token data: {e}"))
    })?;

    // Decompress
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(compressed_data.as_slice());
    let mut json_data = String::new();
    decoder
        .read_to_string(&mut json_data)
        .map_err(|e| Error::SerializationError(format!("Failed to decompress token data: {e}")))?;

    // Parse JSON
    let token_files: std::collections::HashMap<String, String> =
        serde_json::from_str(&json_data)
            .map_err(|e| Error::SerializationError(format!("Failed to parse token JSON: {e}")))?;

    let token_path = Path::new(persist_path);

    // Count files for logging
    let file_count = token_files.len();

    if file_count == 1
        && token_files.keys().next().map(|k| k.as_str())
            == token_path.file_name().and_then(|n| n.to_str())
    {
        // Single file case - write directly to the persist path
        let content = token_files.into_values().next().unwrap();

        // Create parent directory if needed
        if let Some(parent) = token_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                Error::FileIo(format!(
                    "Failed to create token directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        fs::write(token_path, &content)
            .map_err(|e| Error::FileIo(format!("Failed to write token file: {e}")))?;

        // Set secure permissions (600 - owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(token_path)
                .map_err(|e| Error::FileIo(format!("Failed to get file metadata: {e}")))?
                .permissions();
            perms.set_mode(0o600);
            fs::set_permissions(token_path, perms)
                .map_err(|e| Error::FileIo(format!("Failed to set file permissions: {e}")))?;
        }
    } else {
        // Multiple files case - create directory structure
        fs::create_dir_all(token_path).map_err(|e| {
            Error::FileIo(format!(
                "Failed to create token directory {persist_path}: {e}"
            ))
        })?;

        // Write token files
        for (filename, content) in token_files {
            let file_path = token_path.join(&filename);
            fs::write(&file_path, &content).map_err(|e| {
                Error::FileIo(format!("Failed to write token file {filename}: {e}"))
            })?;

            // Set secure permissions (600 - owner read/write only)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&file_path)
                    .map_err(|e| Error::FileIo(format!("Failed to get file metadata: {e}")))?
                    .permissions();
                perms.set_mode(0o600);
                fs::set_permissions(&file_path, perms)
                    .map_err(|e| Error::FileIo(format!("Failed to set file permissions: {e}")))?;
            }
        }
    }

    log::info!("Restored {file_count} token files to {persist_path}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn test_token_export_import_cycle() {
        // Create a temporary directory structure
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let token_dir = temp_dir.path().join("gmail1");
        fs::create_dir_all(&token_dir).expect("Failed to create token dir");

        // Create mock token files
        let mut test_files = HashMap::new();
        test_files.insert(
            "tokencache.json".to_string(),
            r#"{"access_token":"test_access","refresh_token":"test_refresh"}"#.to_string(),
        );
        test_files.insert(
            "metadata.json".to_string(),
            r#"{"created":"2023-01-01","expires":"2023-12-31"}"#.to_string(),
        );

        for (filename, content) in &test_files {
            fs::write(token_dir.join(filename), content).expect("Failed to write test token file");
        }

        // Test export
        let config = crate::ClientConfig::builder()
            .with_client_id("test")
            .with_config_path(temp_dir.path().to_str().unwrap())
            .build();

        // Export tokens (this would normally print to stdout)
        // We'll test the internal function instead
        let result = tokio_test::block_on(export_tokens(&config));
        assert!(result.is_ok(), "Export should succeed");

        // For full integration test, we would capture stdout and test import
        // but that requires more complex setup with process isolation
    }

    #[test]
    fn test_restore_tokens_from_string() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let persist_path = temp_dir.path().join("gmail1").to_string_lossy().to_string();

        // Create test data
        let mut token_data = HashMap::new();
        token_data.insert("test.json".to_string(), r#"{"token":"value"}"#.to_string());

        let json_str = serde_json::to_string(&token_data).unwrap();

        // Compress
        use flate2::Compression;
        use flate2::write::GzEncoder;
        use std::io::Write;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json_str.as_bytes()).unwrap();
        let compressed = encoder.finish().unwrap();

        // Encode
        let encoded = Base64Engine.encode(&compressed);

        // Test restore
        let result = restore_tokens_from_string(&encoded, &persist_path);
        assert!(result.is_ok(), "Restore should succeed: {result:?}");

        // Verify file was created
        let restored_path = Path::new(&persist_path).join("test.json");
        assert!(restored_path.exists(), "Token file should be restored");

        let restored_content = fs::read_to_string(restored_path).unwrap();
        assert_eq!(restored_content, r#"{"token":"value"}"#);
    }

    #[test]
    fn test_missing_token_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = crate::ClientConfig::builder()
            .with_client_id("test")
            .with_config_path(temp_dir.path().join("nonexistent").to_str().unwrap())
            .build();

        let result = tokio_test::block_on(export_tokens(&config));
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::TokenNotFound(_)));
    }

    #[test]
    fn test_invalid_base64_restore() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let persist_path = temp_dir.path().to_string_lossy().to_string();

        let result = restore_tokens_from_string("invalid-base64!", &persist_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::SerializationError(_)));
    }
}
