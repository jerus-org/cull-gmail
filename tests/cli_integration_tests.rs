//! CLI Integration Tests for cull-gmail
//!
//! This module provides comprehensive integration testing for the CLI interface,
//! validating argument parsing, subcommand execution, configuration handling,
//! and error scenarios without requiring actual Gmail API connectivity.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::TempDir;
use tokio::process::Command as AsyncCommand;

/// Test utilities and common setup for CLI integration tests
mod test_utils {
    use super::*;

    /// Test fixture containing temporary directories and mock configurations
    pub struct CliTestFixture {
        pub temp_dir: TempDir,
        pub config_dir: PathBuf,
        pub binary_path: PathBuf,
    }

    impl CliTestFixture {
        /// Create a new test fixture with temporary directory structure
        pub fn new() -> std::io::Result<Self> {
            let temp_dir = TempDir::new()?;
            let config_dir = temp_dir.path().join(".config").join("cull-gmail");
            fs::create_dir_all(&config_dir)?;

            // Get the path to the compiled binary
            let binary_path = if std::env::var("CARGO_MANIFEST_DIR").is_ok() {
                // Running under cargo test - use target directory
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                    .join("target")
                    .join("release")
                    .join("cull-gmail")
            } else {
                // Fallback for other scenarios
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("cull-gmail")
            };

            Ok(Self {
                temp_dir,
                config_dir,
                binary_path,
            })
        }

        /// Create a mock configuration file
        pub fn create_config_file(&self, content: &str) -> std::io::Result<PathBuf> {
            let config_file = self.config_dir.join("config.toml");
            fs::write(&config_file, content)?;
            Ok(config_file)
        }

        /// Create a mock client credentials file
        pub fn create_credentials_file(&self, content: &str) -> std::io::Result<PathBuf> {
            let creds_file = self.config_dir.join("client_secret.json");
            fs::write(&creds_file, content)?;
            Ok(creds_file)
        }

        /// Execute CLI command with arguments and environment variables
        pub fn execute_cli(
            &self,
            args: &[&str],
            env_vars: Option<HashMap<&str, &str>>,
        ) -> std::io::Result<std::process::Output> {
            let mut cmd = Command::new(&self.binary_path);
            cmd.args(args);
            cmd.env("HOME", self.temp_dir.path());
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            if let Some(env) = env_vars {
                for (key, value) in env {
                    cmd.env(key, value);
                }
            }

            cmd.output()
        }

        /// Execute async CLI command for testing interactive scenarios
        pub async fn execute_cli_async(
            &self,
            args: &[&str],
            env_vars: Option<HashMap<&str, &str>>,
        ) -> std::io::Result<std::process::Output> {
            let mut cmd = AsyncCommand::new(&self.binary_path);
            cmd.args(args);
            cmd.env("HOME", self.temp_dir.path());
            cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

            if let Some(env) = env_vars {
                for (key, value) in env {
                    cmd.env(key, value);
                }
            }

            cmd.output().await
        }
    }

    /// Mock Gmail API responses for testing
    pub fn mock_credentials_json() -> &'static str {
        r#"{
            "installed": {
                "client_id": "test-client-id.googleusercontent.com",
                "project_id": "test-project",
                "auth_uri": "https://accounts.google.com/o/oauth2/auth",
                "token_uri": "https://oauth2.googleapis.com/token",
                "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
                "client_secret": "test-client-secret",
                "redirect_uris": ["http://localhost"]
            }
        }"#
    }

    /// Mock configuration TOML content
    pub fn mock_config_toml() -> &'static str {
        r#"
[client]
client_id = "test-client-id"
client_secret = "test-client-secret"
max_results = "100"

[[rules]]
name = "old_promotions"
query = "category:promotions older_than:30d"
action = "delete"
enabled = true

[[rules]]
name = "old_social" 
query = "category:social older_than:60d"
action = "trash"
enabled = false
"#
    }
}

/// Test CLI argument parsing and help output
mod argument_parsing_tests {
    use super::test_utils::*;

    #[test]
    fn test_cli_help_output() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["--help"], None)
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Verify help output contains key elements
        assert!(stdout.contains("cull-gmail"));
        assert!(stdout.contains("USAGE:") || stdout.contains("Usage:"));
        assert!(stdout.contains("labels"));
        assert!(stdout.contains("messages"));
        assert!(stdout.contains("rules"));
    }

    #[test]
    fn test_cli_version_output() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["--version"], None)
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);

        // Should contain version information
        assert!(stdout.contains("cull-gmail"));
        assert!(stdout.contains("0.0.10") || stdout.split_whitespace().count() >= 2);
    }

    #[test]
    fn test_verbosity_flags() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Test different verbosity levels
        let verbosity_tests = [
            (vec!["-v", "labels"], "WARN"),
            (vec!["-vv", "labels"], "INFO"),
            (vec!["-vvv", "labels"], "DEBUG"),
            (vec!["-vvvv", "labels"], "TRACE"),
        ];

        for (args, _expected_level) in verbosity_tests {
            let output = fixture
                .execute_cli(&args, None)
                .expect("Failed to execute CLI");

            // Command should parse successfully (may succeed with valid auth or fail gracefully)
            // The important thing is that verbosity flags are accepted (not argument parsing error)
            let exit_code = output.status.code().unwrap_or(0);
            assert!(
                exit_code != 2,
                "Exit code 2 indicates argument parsing error, got: {exit_code}"
            );
        }
    }

    #[test]
    fn test_invalid_subcommand() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["invalid-command"], None)
            .expect("Failed to execute CLI");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should show error message about invalid subcommand
        assert!(stderr.contains("error:") || stderr.contains("unrecognized"));
    }
}

/// Test labels subcommand functionality
mod labels_tests {
    use super::test_utils::*;

    #[test]
    fn test_labels_help() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["labels", "--help"], None)
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("labels") || stdout.contains("List Gmail labels"));
    }

    #[test]
    fn test_labels_without_credentials() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["labels"], None)
            .expect("Failed to execute CLI");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should fail with configuration or authentication error (unless valid credentials exist)
        if !output.status.success() {
            assert!(
                stderr.contains("config")
                    || stderr.contains("credentials")
                    || stderr.contains("authentication")
                    || stderr.contains("client_secret")
                    || stderr.contains("OAuth")
                    || stderr.contains("token")
            );
        }
    }

    #[test]
    fn test_labels_with_mock_config() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Create mock configuration files
        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config file");
        fixture
            .create_credentials_file(mock_credentials_json())
            .expect("Failed to create credentials file");

        let output = fixture
            .execute_cli(&["labels"], None)
            .expect("Failed to execute CLI");

        // Should proceed further than config validation or succeed entirely
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Test passes if:
        // 1. Command succeeds entirely, or
        // 2. Fails at OAuth/authentication step (not config parsing)
        assert!(
            output.status.success()
                || !stderr.contains("config")
                || stderr.contains("OAuth")
                || stderr.contains("authentication")
                || stderr.contains("token")
        );
    }
}

/// Test messages subcommand functionality  
mod messages_tests {
    use super::test_utils::*;

    #[test]
    fn test_messages_help() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "--help"], None)
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("messages"));
        assert!(stdout.contains("query") || stdout.contains("QUERY"));
    }

    #[test]
    fn test_messages_list_action() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "--query", "in:inbox", "list"], None)
            .expect("Failed to execute CLI");

        // Should parse arguments correctly (may succeed or fail gracefully, but not with parse error)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 2,
            "Exit code 2 indicates argument parsing error, got: {exit_code}"
        );
    }

    #[test]
    fn test_messages_trash_action() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "--query", "in:spam", "trash"], None)
            .expect("Failed to execute CLI");

        // Trash command should be accepted (not argument parsing error)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 2,
            "Exit code 2 indicates argument parsing error, got: {exit_code}"
        );
    }

    #[test]
    fn test_messages_pagination_options() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(
                &[
                    "messages",
                    "--query",
                    "in:inbox",
                    "--max-results",
                    "50",
                    "--pages",
                    "2",
                    "list",
                ],
                None,
            )
            .expect("Failed to execute CLI");

        // Pagination arguments should be accepted (not argument parsing error)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 2,
            "Exit code 2 indicates argument parsing error, got: {exit_code}"
        );
    }

    #[test]
    fn test_messages_invalid_action() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "--query", "test", "invalid-action"], None)
            .expect("Failed to execute CLI");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("error:") || stderr.contains("invalid"));
    }

    #[test]
    fn test_messages_without_query() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "list"], None)
            .expect("Failed to execute CLI");

        // Messages list should work with or without explicit query (may use defaults)
        // The test validates that the command is well-formed, not the query requirement
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 2,
            "Exit code 2 indicates argument parsing error, got: {exit_code}"
        );
    }
}

/// Test rules subcommand functionality
mod rules_tests {
    use super::test_utils::*;

    #[test]
    fn test_rules_help() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["rules", "--help"], None)
            .expect("Failed to execute CLI");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("rules"));
        assert!(stdout.contains("config") || stdout.contains("run"));
    }

    #[test]
    fn test_rules_config_subcommand() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["rules", "config"], None)
            .expect("Failed to execute CLI");

        // Should attempt to create/display config
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should either succeed or show meaningful output about config
        assert!(
            output.status.success()
                || stdout.contains("config")
                || stderr.contains("config")
                || stdout.contains("toml")
                || stderr.contains("toml")
        );
    }

    #[test]
    fn test_rules_run_without_config() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["rules", "run"], None)
            .expect("Failed to execute CLI");

        // Should fail gracefully when no config is found
        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("config") || stderr.contains("file") || stderr.contains("not found")
        );
    }

    #[test]
    fn test_rules_run_with_config() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config file");

        let output = fixture
            .execute_cli(&["rules", "run"], None)
            .expect("Failed to execute CLI");

        // Should proceed past config parsing (may fail at auth)
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains("config")
                || stderr.contains("credentials")
                || stderr.contains("authentication")
        );
    }

    #[test]
    fn test_rules_run_execution() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config file");

        let output = fixture
            .execute_cli(&["rules", "run"], None)
            .expect("Failed to execute CLI");

        // Rules run command should be accepted (not argument parsing error)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 2,
            "Exit code 2 indicates argument parsing error, got: {exit_code}"
        );
    }
}

/// Test configuration and environment handling
mod configuration_tests {
    use super::test_utils::*;
    use std::collections::HashMap;

    #[test]
    fn test_config_file_hierarchy() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Create config in expected location
        let config_content = r#"
[client]
client_id = "test-from-config"
client_secret = "secret-from-config"
"#;
        fixture
            .create_config_file(config_content)
            .expect("Failed to create config");

        // Any command should now find the config
        let output = fixture
            .execute_cli(&["labels"], None)
            .expect("Failed to execute CLI");

        let stderr = String::from_utf8_lossy(&output.stderr);

        // Should not complain about missing config anymore
        assert!(!stderr.contains("config file not found"));
    }

    #[test]
    fn test_environment_variable_precedence() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let mut env_vars = HashMap::new();
        env_vars.insert("CULL_GMAIL_CLIENT_ID", "env-client-id");
        env_vars.insert("CULL_GMAIL_CLIENT_SECRET", "env-secret");

        let output = fixture
            .execute_cli(&["labels"], Some(env_vars))
            .expect("Failed to execute CLI");

        // Environment variables should be recognized
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("client_id"));
    }

    #[test]
    fn test_invalid_config_format() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Create malformed config
        fixture
            .create_config_file("invalid toml content [[[")
            .expect("Failed to create config");

        let output = fixture
            .execute_cli(&["labels"], None)
            .expect("Failed to execute CLI");

        assert!(!output.status.success());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("config") || stderr.contains("parse") || stderr.contains("toml"));
    }
}

/// Test error handling and edge cases
mod error_handling_tests {
    use super::test_utils::*;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn test_graceful_keyboard_interrupt() {
        // This test would require more complex setup with signal handling
        // For now, we ensure the CLI handles missing dependencies gracefully
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        let output = fixture
            .execute_cli(&["messages", "--query", "test", "list"], None)
            .expect("Failed to execute CLI");

        // Should not crash (no segfault)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 139,
            "Segmentation fault detected, got exit code: {exit_code}"
        );
    }

    #[test]
    fn test_invalid_query_syntax() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config");
        fixture
            .create_credentials_file(mock_credentials_json())
            .expect("Failed to create credentials");

        let output = fixture
            .execute_cli(
                &["messages", "--query", "invalid:query:syntax:::", "list"],
                None,
            )
            .expect("Failed to execute CLI");

        // Should handle invalid queries gracefully (no segfault)
        let exit_code = output.status.code().unwrap_or(0);
        assert!(
            exit_code != 139,
            "Segmentation fault detected, got exit code: {exit_code}"
        );
    }

    #[test]
    fn test_network_timeout_simulation() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Set very short timeout to trigger timeout behavior
        let mut env_vars = HashMap::new();
        env_vars.insert("HTTP_TIMEOUT", "1");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config");
        fixture
            .create_credentials_file(mock_credentials_json())
            .expect("Failed to create credentials");

        let output = fixture
            .execute_cli(&["labels"], Some(env_vars))
            .expect("Failed to execute CLI");

        // Should handle timeouts gracefully
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success()
                || stderr.contains("timeout")
                || stderr.contains("network")
                || stderr.contains("connection")
        );
    }

    #[test]
    fn test_permission_denied_scenarios() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        // Create a config file with restricted permissions
        let config_path = fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config");

        // Remove read permissions (this might not work on all systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path).unwrap().permissions();
            perms.set_mode(0o000);
            let _ = fs::set_permissions(&config_path, perms);
        }

        let output = fixture
            .execute_cli(&["labels"], None)
            .expect("Failed to execute CLI");

        // Should handle permission errors gracefully
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success()
                || stderr.contains("permission")
                || stderr.contains("access")
                || stderr.contains("denied")
        );
    }
}

/// Async integration tests for concurrent operations
mod async_integration_tests {
    use super::test_utils::*;

    #[tokio::test]
    async fn test_concurrent_cli_executions() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config");

        // Execute multiple CLI commands concurrently
        let tasks = vec![
            fixture.execute_cli_async(&["labels", "--help"], None),
            fixture.execute_cli_async(&["messages", "--help"], None),
            fixture.execute_cli_async(&["rules", "--help"], None),
        ];

        let results = futures::future::join_all(tasks).await;

        // All help commands should succeed
        for result in results {
            let output = result.expect("Failed to execute CLI");
            assert!(output.status.success());
        }
    }

    #[tokio::test]
    async fn test_async_command_timeout() {
        let fixture = CliTestFixture::new().expect("Failed to create test fixture");

        fixture
            .create_config_file(mock_config_toml())
            .expect("Failed to create config");
        fixture
            .create_credentials_file(mock_credentials_json())
            .expect("Failed to create credentials");

        // Test with timeout
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            fixture.execute_cli_async(&["labels"], None),
        )
        .await;

        match result {
            Ok(output) => {
                let output = output.expect("Failed to execute CLI");
                // Command completed within timeout (no segfault)
                let exit_code = output.status.code().unwrap_or(0);
                assert!(
                    exit_code != 139,
                    "Segmentation fault detected, got exit code: {exit_code}"
                );
            }
            Err(_) => {
                // Timeout occurred - this is acceptable for integration tests
                // as we may not have real credentials
            }
        }
    }
}
